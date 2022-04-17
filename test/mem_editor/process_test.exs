defmodule MemEditor.ProcessTest do
  use ExUnit.Case
  doctest MemEditor.Process

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

  test ".with_name() sees the Calculator", %{calc_pid: os_pid} do
    assert {:ok, proc} = MemEditor.Process.with_name("Calculator")
    assert os_pid == proc.pid
  end

  test ".attach() can attach to the Calculator" do
    {:ok, proc} = MemEditor.Process.with_name("Calculator")
    assert {:ok, status} = MemEditor.Process.attach(proc)
  end

  test ".continue() continues an attached process" do
    {:ok, proc} = MemEditor.Process.with_name("Calculator")
    assert :ok = MemEditor.Process.attach(proc)
    assert {:ok, status} = MemEditor.Process.continue(proc)
  end

  test ".continue() fails to an unattached process" do
    {:ok, proc} = MemEditor.Process.with_name("Calculator")
    assert {:error, _} = MemEditor.Process.continue(proc)
  end

  test ".task() returns a task" do
    {:ok, proc} = MemEditor.Process.with_name("Calculator")
    assert {:ok, %MemEditor.Task{port: _}} = MemEditor.Process.task(proc)
  end
end
