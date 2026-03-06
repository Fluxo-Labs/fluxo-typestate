use fluxo_typestate::state_machine;

#[state_machine]
enum Computer {
    #[transition(Computer::Idle -> Computer::Running: start)]
    #[transition(Computer::Idle -> Computer::Sleeping: sleep)]
    Idle,
    #[transition(Computer::Running -> Computer::Idle: stop)]
    #[transition(Computer::Running -> Computer::Sleeping: suspend)]
    Running { cpu_load: f32 },
    #[transition(Computer::Sleeping -> Computer::Idle: wake)]
    Sleeping,
}

fn main() {
    let computer: Computer<Idle> = Computer::new();
    println!("Initial state: {}", computer.current_state());

    let running: Computer<Running> = computer.start();
    println!(
        "Started computer with CPU load: {}",
        running._inner_running.cpu_load
    );

    let sleeping: Computer<Sleeping> = running.suspend();
    println!("Computer is now sleeping");

    let idle: Computer<Idle> = sleeping.wake();
    println!("Computer woke up, state: {}", idle.current_state());

    println!(
        "\nMermaid Diagram:\n{}",
        Computer::<Idle>::mermaid_diagram()
    );
}
