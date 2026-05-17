mod adapters;
mod detector;

use adapters::AgentAdapter;
use colored::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::thread;

fn main() {
    println!();
    println!(
        "{}",
        "  ╔═══════════════════════════════════════╗".bright_green()
    );
    println!(
        "{}",
        "  ║          devgrace report              ║"
            .bright_green()
            .bold()
    );
    println!(
        "{}",
        "  ╚═══════════════════════════════════════╝".bright_green()
    );
    println!();

    let adapters: Vec<Box<dyn AgentAdapter>> = vec![
        Box::new(adapters::claude::ClaudeAdapter),
        Box::new(adapters::codex::CodexAdapter),
        Box::new(adapters::opencode::OpenCodeAdapter),
        Box::new(adapters::cline::ClineAdapter),
        Box::new(adapters::amp::AmpAdapter),
        Box::new(adapters::pi::PiAdapter),
        Box::new(adapters::zed::ZedAdapter),
    ];

    // Run all adapters in parallel threads
    let handles: Vec<_> = adapters
        .into_iter()
        .map(|adapter| {
            thread::spawn(move || {
                let name = adapter.name().to_string();
                let msgs = adapter.extract_messages();
                let texts: Vec<String> = msgs.iter().map(|m| m.text.clone()).collect();
                (name, texts)
            })
        })
        .collect();

    let mut all_messages: Vec<String> = Vec::new();
    let mut agent_messages: HashMap<String, Vec<String>> = HashMap::new();

    for handle in handles {
        let (name, texts) = handle.join().unwrap();
        if !texts.is_empty() {
            all_messages.extend(texts.clone());
            agent_messages.insert(name, texts);
        }
    }

    if all_messages.is_empty() {
        println!(
            "{}",
            "  No messages found. Make sure you've used some coding agents!".yellow()
        );
        println!();
        return;
    }

    let total_messages = all_messages.len();
    let (total_politeness, all_counts) = detector::count_politeness(&all_messages);

    println!(
        "  {} {}",
        "Messages scanned:".white().bold(),
        total_messages.to_string().bright_cyan()
    );
    println!(
        "  {} {}",
        "Total politeness:".white().bold(),
        total_politeness.to_string().bright_green().bold()
    );
    println!();

    let grace_score = if total_messages > 0 {
        (total_politeness as f64 / total_messages as f64) * 100.0
    } else {
        0.0
    };

    let grace_label = if grace_score >= 10.0 {
        "Certified Sweetheart".bright_green().bold()
    } else if grace_score >= 5.0 {
        "Genuinely Kind".bright_green()
    } else if grace_score >= 2.0 {
        "Polite Enough".bright_yellow()
    } else if grace_score >= 0.5 {
        "Could Be Nicer".yellow()
    } else {
        "Emotionally Neutral".bright_red()
    };

    println!(
        "  {} {:.1}%  {}",
        "Grace score:".white().bold(),
        grace_score,
        format!("← {}", grace_label).dimmed()
    );
    println!();

    // By agent — parallel politeness counting
    println!("{}", "  By agent:".white().bold().underline());
    println!();

    let mut agent_stats: Vec<(String, usize, usize, f64)> = agent_messages
        .par_iter()
        .map(|(name, msgs)| {
            let count = msgs.len();
            let (polite, _) = detector::count_politeness(msgs);
            let pct = if count > 0 {
                polite as f64 / count as f64 * 100.0
            } else {
                0.0
            };
            (name.clone(), count, polite, pct)
        })
        .collect();
    agent_stats.sort_by(|a, b| b.2.cmp(&a.2));

    for (name, count, polite, pct) in &agent_stats {
        let agent_colored = match name.as_str() {
            "claude" => name.bright_magenta(),
            "codex" => name.bright_blue(),
            "opencode" => name.bright_cyan(),
            "cline" => name.bright_green(),
            "amp" => name.bright_yellow(),
            "pi" => name.bright_white(),
            "zed" => name.bright_red(),
            _ => name.white(),
        };
        println!(
            "  {} {} {} {} {}",
            format!("  • {:<10}", agent_colored).white(),
            polite.to_string().bright_green().bold(),
            "politeness in".dimmed(),
            format!("{} messages", count).bright_cyan(),
            format!("({:.1}%)", pct).dimmed(),
        );
    }
    println!();

    // Top words
    let top = detector::top_words(&all_counts, 10);
    if !top.is_empty() {
        println!("{}", "  Top polite words:".white().bold().underline());
        println!();
        for item in &top {
            let bar_len = (item.count as f64).sqrt() as usize;
            let bar: String = "█".repeat(bar_len.min(20));
            println!(
                "  {} {} {}",
                format!("  • {:<20}", item.word).white(),
                item.count.to_string().bright_green().bold(),
                bar.bright_green()
            );
        }
    }

    println!();
    println!(
        "{}",
        "  ═══════════════════════════════════════════".bright_green()
    );
    println!(
        "{}",
        "  Stay kind to your machines. They remember."
            .bright_green()
            .dimmed()
    );
    println!(
        "{}",
        "  ═══════════════════════════════════════════".bright_green()
    );
    println!();
}
