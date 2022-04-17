#[rustler::nif]

fn attach(pid: u32) {
    println!("{:?}", std::env::current_exe());
    unsafe {
        let mut addr: i8 = 0;
        let resp = ::libc::ptrace(::libc::PT_ATTACH, pid as ::libc::pid_t, &mut addr, 0);
        let errno = ::std::io::Error::last_os_error().raw_os_error();
        let mut task_port: ::mach::port::mach_port_name_t = 0;

        let err = ::mach::traps::task_for_pid(mach::traps::mach_task_self(), pid as ::libc::c_int, &mut task_port);
        println!("task_for_pid: {}", err);

        match errno {
            None => println!("PTRACE of {}: response: {}", pid, resp),
            Some(::libc::ESRCH) => println!("PTRACE of {}: response: {}, errno: {}", pid, resp, "ESRCH"),
            Some(::libc::EINVAL) => println!("PTRACE of {}: response: {}, errno: {}", pid, resp, "EINVAL"),
            Some(::libc::EBUSY) => println!("PTRACE of {}: response: {}, errno: {}", pid, resp, "EBUSY"),
            Some(::libc::EPERM) => println!("PTRACE of {}: response: {}, errno: {}", pid, resp, "EPERM"),
            Some(err) => println!("PTRACE of {}: response: {}, errno: {}", pid, resp, err),
        }
    }
}

// fn add(a: i64, b: i64) -> i64 {
//     a + b
// }
rustler::init!("Elixir.MemEditor.Editor", [attach]);
