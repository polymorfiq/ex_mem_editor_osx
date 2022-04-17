defmodule MemEditor.Backends.Osx do
  use Rustler, otp_app: :mem_editor, crate: "backends_osx"

  @type os_pid :: non_neg_integer()

  @spec ptrace_attach(os_pid()) :: :ok | {:error, any()}
  def ptrace_attach(_pid), do: error()

  @spec ptrace_detach(os_pid()) :: :ok | {:error, any()}
  def ptrace_detach(_pid), do: error()

  @spec ptrace_continue(os_pid(), integer(), integer()) :: :ok | {:error, any()}
  def ptrace_continue(_pid, _addr, _data), do: error()

  @spec list_pids() :: {:ok, [{os_pid(), String.t()}]} | {:error, any()}
  def list_pids(), do: error()

  @spec task_for_pid(os_pid()) :: {:ok, {non_neg_integer()}} | {:error, any()}
  def task_for_pid(_pid), do: error()

  @spec wait_pid(os_pid(), integer()) :: {:ok, integer()}, {:error, any}
  def wait_pid(_pid, _opts), do: error()

  defp error(), do: :erlang.nif_error(:nif_not_loaded)
end
