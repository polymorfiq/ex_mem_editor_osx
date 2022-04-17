use rustler::{Encoder, Env, Term};

rustler::atoms! { error, ok, }

type PID = ::libc::pid_t;
type Port = ::libc::c_uint;

struct AttachedProcess {
    pid: PID
}

impl<'a> Encoder for AttachedProcess {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        (self.pid).encode(env)
    }
}

struct Task {
    port: Port
}

impl<'a> Encoder for Task {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        (self.port).encode(env)
    }
}

enum ListPidResult {
    Success(Vec<(PID, String)>),
    Failure(String),
}

impl<'a> Encoder for ListPidResult {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        match self {
            ListPidResult::Success(list) => (ok(), list).encode(env),
            ListPidResult::Failure(msg) => (error(), msg).encode(env),
        }
    }
}

#[rustler::nif]
fn list_pids() -> ListPidResult {
    match darwin_libproc::all_pids() {
        Ok(pids) => {
            let list = pids.into_iter().map (|pid| {
                let unknown = String::from("(Unknown name)");
                match get_pid_name(pid) {
                    Ok(name) => (pid, name),
                    Err(_err) => (pid, unknown),
                }
            }).collect::<Vec<_>>();
    
            ListPidResult::Success(list)
        }

        Err(err) => ListPidResult::Failure(format!("darwin_libproc::all_pids failed: {}", err))
    }
}

fn get_pid_name(pid: PID) -> Result<String, String> {
    let unknown_name = String::from("(Unknown name)");

    match darwin_libproc::name(pid as PID) {
        Ok(name) => {
            match name.into_string() {
                Ok(name_string) => Ok(name_string),
                Err(_) => Err(unknown_name)
            }
        },
        Err(_) => Err(unknown_name)
    }
    
}

enum TaskResult {
    Success(Task),
    Failure(String),
}

impl<'a> Encoder for TaskResult {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        match self {
            TaskResult::Success(task) => (ok(), task).encode(env),
            TaskResult::Failure(msg) => (error(), msg).encode(env),
        }
    }
}

#[rustler::nif]
fn task_for_pid(pid: PID) -> TaskResult {
    let mut task_port: ::mach::port::mach_port_name_t = 0;

    unsafe {
        let err = ::mach::traps::task_for_pid(mach::traps::mach_task_self(), pid as ::libc::c_int, &mut task_port);
        if err == 0 {
            TaskResult::Success(Task{port: task_port})
        } else {
            TaskResult::Failure(format!("task_for_pid failed: {}", err))
        }
    }
}

enum ProcessAttachResult {
    Success(AttachedProcess),
    Failure(String),
}

impl<'a> Encoder for ProcessAttachResult {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        match self {
            ProcessAttachResult::Success(_proc) => ok().encode(env),
            ProcessAttachResult::Failure(msg) => (error(), msg).encode(env),
        }
    }
}

#[rustler::nif]
fn ptrace_attach(pid: PID) -> ProcessAttachResult {
    unsafe {
        let mut addr: i8 = 0;
        let trace_resp = ::libc::ptrace(::libc::PT_ATTACH, pid as PID, &mut addr, 0);
        let errno = ::std::io::Error::last_os_error().raw_os_error();

        match (trace_resp, errno) {
            (0, _) => ProcessAttachResult::Success(AttachedProcess{pid: pid}),
            (-1, None) => ProcessAttachResult::Failure(format!("PTRACE_ATTACH of {}: ptrace response: {}", pid, trace_resp)),
            (-1, Some(::libc::ESRCH)) => ProcessAttachResult::Failure(format!("PTRACE_ATTACH of {}: response: {}, errno: {}", pid, trace_resp, "ESRCH")),
            (-1, Some(::libc::EINVAL)) => ProcessAttachResult::Failure(format!("PTRACE_ATTACH of {}: response: {}, errno: {}", pid, trace_resp, "EINVAL")),
            (-1, Some(::libc::EBUSY)) => ProcessAttachResult::Failure(format!("PTRACE_ATTACH of {}: response: {}, errno: {}", pid, trace_resp, "EBUSY")),
            (-1, Some(::libc::EPERM)) => ProcessAttachResult::Failure(format!("PTRACE_ATTACH of {}: response: {}, errno: {}", pid, trace_resp, "EPERM")),
            (-1, Some(err)) => ProcessAttachResult::Failure(format!("PTRACE_ATTACH of {}: response: {}, errno: {}", pid, trace_resp, err)),
            (resp_code, Some(err)) => ProcessAttachResult::Failure(format!("PTRACE_ATTACH of {}: unexpected response code: {}, errno: {}", pid, resp_code, err)),
            (resp_code, None) => ProcessAttachResult::Failure(format!("PTRACE_ATTACH of {}: unexpected response code: {}", pid, resp_code)),
        }
    }
}

#[rustler::nif]
fn ptrace_detach(pid: PID) -> ProcessAttachResult {
    unsafe {
        let mut addr: i8 = 0;
        let trace_resp = ::libc::ptrace(::libc::PT_DETACH, pid as PID, &mut addr, 0);
        let errno = ::std::io::Error::last_os_error().raw_os_error();

        match (trace_resp, errno) {
            (0, _) => ProcessAttachResult::Success(AttachedProcess{pid: pid}),
            (-1, None) => ProcessAttachResult::Failure(format!("PTRACE_ATTACH of {}: ptrace response: {}", pid, trace_resp)),
            (-1, Some(::libc::ESRCH)) => ProcessAttachResult::Failure(format!("PTRACE_ATTACH of {}: response: {}, errno: {}", pid, trace_resp, "ESRCH")),
            (-1, Some(::libc::EINVAL)) => ProcessAttachResult::Failure(format!("PTRACE_ATTACH of {}: response: {}, errno: {}", pid, trace_resp, "EINVAL")),
            (-1, Some(::libc::EBUSY)) => ProcessAttachResult::Failure(format!("PTRACE_ATTACH of {}: response: {}, errno: {}", pid, trace_resp, "EBUSY")),
            (-1, Some(::libc::EPERM)) => ProcessAttachResult::Failure(format!("PTRACE_ATTACH of {}: response: {}, errno: {}", pid, trace_resp, "EPERM")),
            (-1, Some(err)) => ProcessAttachResult::Failure(format!("PTRACE_ATTACH of {}: response: {}, errno: {}", pid, trace_resp, err)),
            (resp_code, Some(err)) => ProcessAttachResult::Failure(format!("PTRACE_ATTACH of {}: unexpected response code: {}, errno: {}", pid, resp_code, err)),
            (resp_code, None) => ProcessAttachResult::Failure(format!("PTRACE_ATTACH of {}: unexpected response code: {}", pid, resp_code)),
        }
    }
}

#[rustler::nif]
fn wait_pid(pid: PID, opts: i32) -> (rustler::Atom, i32) {
    let mut wstatus: i32 = 1;
    unsafe { ::libc::waitpid(pid, &mut wstatus, opts); }
    (ok(), wstatus)
}

#[rustler::nif]
fn ptrace_continue(pid: PID, start_addr: i8, data: i32) -> ProcessAttachResult {
    unsafe {
        let mut addr: i8 = start_addr;
        let trace_resp = ::libc::ptrace(::libc::PT_CONTINUE, pid as PID, &mut addr, data);
        let errno = ::std::io::Error::last_os_error().raw_os_error();

        match (trace_resp, errno) {
            (0, _) => ProcessAttachResult::Success(AttachedProcess{pid: pid}),
            (-1, None) => ProcessAttachResult::Failure(format!("PTRACE_CONTINUE of {}: ptrace response: {}", pid, trace_resp)),
            (-1, Some(::libc::ESRCH)) => ProcessAttachResult::Failure(format!("PTRACE_CONTINUE of {}: response: {}, errno: {}", pid, trace_resp, "ESRCH")),
            (-1, Some(::libc::EINVAL)) => ProcessAttachResult::Failure(format!("PTRACE_CONTINUE of {}: response: {}, errno: {}", pid, trace_resp, "EINVAL")),
            (-1, Some(::libc::EBUSY)) => ProcessAttachResult::Failure(format!("PTRACE_CONTINUE of {}: response: {}, errno: {}", pid, trace_resp, "EBUSY")),
            (-1, Some(::libc::ENOTSUP)) => ProcessAttachResult::Failure(format!("PTRACE_CONTINUE of {}: response: {}, errno: {}", pid, trace_resp, "ENOTSUP")),
            (-1, Some(::libc::EPERM)) => ProcessAttachResult::Failure(format!("PTRACE_CONTINUE of {}: response: {}, errno: {}", pid, trace_resp, "EPERM")),
            (-1, Some(err)) => ProcessAttachResult::Failure(format!("PTRACE_CONTINUE of {}: response: {}, errno: {}", pid, trace_resp, err)),
            (resp_code, Some(err)) => ProcessAttachResult::Failure(format!("PTRACE_CONTINUE of {}: unexpected response code: {}, errno: {}", pid, resp_code, err)),
            (resp_code, None) => ProcessAttachResult::Failure(format!("PTRACE_CONTINUE of {}: unexpected response code: {}", pid, resp_code)),
        }
    }
}

rustler::init!("Elixir.MemEditor.Backends.Osx", [
    list_pids,
    ptrace_attach,
    ptrace_detach,
    ptrace_continue,
    wait_pid,
    task_for_pid
]);
