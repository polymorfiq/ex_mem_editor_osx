defmodule MemEditor.Editor do
  use Rustler, otp_app: :mem_editor, crate: "mem_editor_editor"

  @spec attach(integer()) :: nil
  def attach(_x), do: error()

  defp error(), do: :erlang.nif_error(:nif_not_loaded)
end
