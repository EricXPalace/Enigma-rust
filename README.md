# Overview
This project is targeting to make a Enigma machine in RUST language in learning purpose.

# Usage
By current plan, this project will be an commandline tool in following usage:
~~~
./enigma <filename> --dial <dial-cog-number> --wire <wiretab-pair>
~~~
To simulate the real enigma machine on history.

# Develop Plan
1. Understanding how Enigma works.
2. **(CURRENT STATE)** Build main.rs as main mechanism. // ./enigma <string>
3. Add dial function. // ./enigma <string> --dial <dial-cog-number>
4. Add wire function. // ./enigma <string> --dial <dial-cog-number> --wire <wiretab-pair>
5. Add file encrypt function.
6. Add keyboard shifting function
7. Enter maintenance state (End of updation)

# Workflow
* 2023-11-02 Downgrade [clap](https://crates.io/crates/clap/versions) to meet rustc version on [Replit](replit.com).
* 2023-11-01 Add clap crate as main crate for developing.
* 2023-11-01 Start Project