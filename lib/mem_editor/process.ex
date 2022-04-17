defmodule MemEditor.Process do
  alias MemEditor.Backends.Osx
  alias MemEditor.Task
  @enforce_keys [:pid]
  defstruct [:pid, :name]
  @wnohang 0x00000001
  @wuntraced 0x00000002

  @type t :: %__MODULE__{
    pid: non_neg_integer(),
    name: String.t() | nil
  }

  @spec with_name(String.t() | Regex.t()) :: {:ok, {non_neg_integer(), String.t()}} | {:error, any}
  def with_name(search) do
    match = case Osx.list_pids() do
      {:ok, pids} ->
        {:ok, pids |> Enum.find(fn {_, name} -> name =~ search end)}

      {:error, err} -> {:error, "Could not list processes: #{err}"}
    end

    with {:ok, info} <- match do
      case info do
        {pid, name} -> {:ok, %__MODULE__{pid: pid, name: name}}
        nil -> {:error, "No matching process found"}
      end
    end
  end

  @spec task(t()) :: {:ok, Task.t()} | {:error, any()}
  def task(proc) do
    Task.for_pid(proc.pid)
  end

  @spec attach(t()) :: {:ok, integer()} | {:error, any}
  def attach(proc) do
    with :ok <- Osx.ptrace_attach(proc.pid) do
      Osx.wait_pid(proc.pid, @wnohang)
    end
  end

  @spec detach(t()) :: {:ok, integer()} | {:error, any}
  def detach(proc) do
    with :ok <- Osx.ptrace_detach(proc.pid) do
      Osx.wait_pid(proc.pid, @wnohang)
    end
  end

  @spec continue(t(), integer(), integer()) :: {:ok, integer()} | {:error, any}
  def continue(proc, addr \\ 1, data \\ 0) do
    with :ok <- Osx.ptrace_continue(proc.pid, addr, data) do
      Osx.wait_pid(proc.pid, @wnohang)
    end
  end
end
