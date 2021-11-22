defmodule PenmarkWeb.SudokuLive do
  use Phoenix.LiveView

  import PenmarkWeb.SudokuGrid
  import PenmarkWeb.SudokuControls

  def render(assigns) do
    ~H"""
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div class="flex max-w-7xl mx-auto h-screen space-x-40 items-center">
    <div class="w-2/3">
    <.grid sudoku={@sudoku} />
    </div>
    <div class="w-1/3">
    <.controls selected_number={@selected_number} />
    </div>
    </div>
    </div>
    """
  end

  def get_empty_grid() do
    for _ <- 1..9 do
      for _ <- 1..9 do
        nil
      end
    end
  end

  def get_grid() do
    for _ <- 1..9 do
      for _ <- 1..9 do
        if :rand.uniform(2) == 1 do
          :rand.uniform(9)
        else
          nil
        end
      end
    end
  end

  def mount(_params, %{}, socket) do
    socket = assign(socket, :selected_number, nil)

    if not connected?(socket) do
      socket =
        socket
        |> assign(:sudoku, get_empty_grid())

      {:ok, socket}
    else
      socket =
        socket
        |> assign(:sudoku, get_grid())

      {:ok, socket}
    end
  end

  def handle_event("select_cell", %{"cell" => [x, y]}, socket) do
    if is_nil(socket.assigns.selected_number) do
      {:noreply, socket}
    else
      sudoku = put_in(socket.assigns.sudoku, [Access.at(x),Access.at(y)], socket.assigns.selected_number)
      socket = assign(socket, :sudoku, sudoku)
      {:noreply, socket}
    end
  end

  def handle_event("select_number", %{"number" => number}, socket) do
    socket = assign(socket, :selected_number, number)
    {:noreply, socket}
  end
end
