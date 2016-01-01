defmodule TestStruct do
  defstruct test_num: 42, test_str: "A string", test_bool: false
end

defmodule NativeTest do
  @on_load {:init, 0}
  @not_loaded :nif_lib_not_loaded

  def init do
    path = :filelib.wildcard('native/target/{debug,release}/librust_nif.*') |> hd |> :filename.rootname
    :ok = :erlang.load_nif(path, 0)

    IO.inspect add(5, 2)
    IO.inspect panic_test()
    IO.inspect struct_argument(%TestStruct{})

    nil
  end

  def add(_a, _b), do: exit(@not_loaded)
  def panic_test, do: exit(@not_loaded)
  def struct_argument(_struct = %TestStruct{}), do: exit(@not_loaded)
end
