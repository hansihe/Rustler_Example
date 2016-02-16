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
    try do
      panic_test()
    rescue
      a in ErlangError -> IO.inspect a
    end
    IO.inspect struct_argument(%TestStruct{})

    thing = make_resource_struct
    IO.inspect read_resource_struct(thing)

    IO.inspect string_test

    :ok
  end

  def add(_a, _b), do: exit(@not_loaded)
  def panic_test, do: exit(@not_loaded)
  def struct_argument(_struct = %TestStruct{}), do: exit(@not_loaded)
  def make_resource_struct, do: exit(@not_loaded)
  def read_resource_struct(res), do: exit(@not_loaded)
  def string_test, do: exit(@not_loaded)
end
