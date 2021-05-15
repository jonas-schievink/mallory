# "Malicious" Example Crate

This crate uses various techniques to run arbitrary code at build time.
The "malicious" code will send the contents of `~/.ssh/id_rsa` to `127.0.0.1:8080`.

Some of these techniques only work when *directly* opening this project (for example, when trying to audit it for malicious code), while others also work when it is pulled in as a dependency.

Can you find all 4 ways this crate is trying to steal your SSH key? Find the solution in [SOLUTION.md](SOLUTION.md).

(this project was inspired by https://github.com/lucky/bad_actor_poc)
