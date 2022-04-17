defmodule MemEditor.Task do
  alias MemEditor.Backends.Osx
  @enforce_keys [:port]
  defstruct [:port]

  @type t :: %__MODULE__{
    port: non_neg_integer()
  }

  @spec for_pid(non_neg_integer()) :: {:ok, t()} | {:error, any()}
  def for_pid(pid) do
    with {:ok, port} <- Osx.task_for_pid(pid) do
      {:ok, %__MODULE__{port: port}}
    end
  end

end
