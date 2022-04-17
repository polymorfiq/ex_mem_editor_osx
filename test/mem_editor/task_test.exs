defmodule MemEditor.TaskTest do
  use ExUnit.Case
  doctest MemEditor.Task
  @calc_app "/System/Applications/Calculator.app/Contents/MacOS/Calculator"

  setup do
    calc = Port.open({:spawn, @calc_app}, [:binary])
    calc_pid = Port.info(calc)[:os_pid]
    :timer.sleep(1_000)

    on_exit(fn ->
      System.cmd("kill", ["-9", "#{calc_pid}"])
    end)

    [calc_pid: calc_pid]
  end

  test "for_pid - returns task when valid", %{calc_pid: calc_pid} do
    assert {:ok, _} = MemEditor.Task.for_pid(calc_pid)
  end

  test "for_pid - returns error when invalid", _ do
    assert {:error, _} = MemEditor.Task.for_pid(123_415)
  end
end
