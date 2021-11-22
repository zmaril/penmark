defmodule PenmarkWeb.SudokuGrid do
  # If you generated an app with mix phx.new --live,
  # the line below would be: use MyAppWeb, :live_component
  use Phoenix.LiveComponent
  alias Phoenix.LiveView.JS

  defp svg_grid() do
  end

  def grid(assigns) do
    ~H"""
    <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" height="auto" width="auto">

      <g id="grid" >
      <rect x="0" width="100" height="100" style="stroke:rgb(0,0,0);stroke-width:3; fill:none" />

      <line x1="0" y1="33" x2="100" y2="33" style="stroke:rgb(0,0,0);stroke-width:1"/>
      <line x1="0" y1="66" x2="100" y2="66" style="stroke:rgb(0,0,0);stroke-width:1"/>

      <line x1="33" y1="0" x2="33" y2="100" style="stroke:rgb(0,0,0);stroke-width:1"/>
      <line x1="66" y1="0" x2="66" y2="100" style="stroke:rgb(0,0,0);stroke-width:1"/>


      <%= for n <- 0..9, rem(n,3) != 0 do %>
      <!--<line x1={n * 100/9} y1="0" x2={n*100/9} y2="100" style="stroke:rgb(0,0,0);stroke-width:0.2" /> -->
      <!-- <line x1="0" y1={n * 100/9} x2="600" y2={n*100/9} style="stroke:rgb(0,0,0);stroke-width:0.2" /> -->
      <% end %>
      </g>

      <g id="givens">
      <%= for x <- 0..8 do %>
      <%= for y <- 0..8 do %>
      <text font-size="8"
      fill="black"
      font-family="Verdana"
      text-anchor="middle"
      alignment-baseline="central"
            x={x*100/9 + 5.5}
      y={y*100/9 + 8.5}> <%= assigns.sudoku |> Enum.at(x) |> Enum.at(y) %></text>
      <rect x={x * 100/9} y={y * 100/9}
      phx-click={JS.push("select_cell", value: %{"cell" => [x,y]})}

      width="11" height="11" style="stroke:rgb(0,0,0);stroke-width:0.2; fill-opacity:0" />
      <% end %>
      <% end %>
      </g>

    </svg>
    """
  end
end
