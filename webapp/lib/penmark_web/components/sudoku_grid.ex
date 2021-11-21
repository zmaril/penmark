defmodule PenmarkWeb.SudokuGrid do
  # If you generated an app with mix phx.new --live,
  # the line below would be: use MyAppWeb, :live_component
  use Phoenix.LiveComponent

  defp svg_grid() do
  end

  def grid(assigns) do
    ~H"""
    <svg height="600" width="600">
      <g id="grid" >
      <rect x="0" width="600" height="600" style="stroke:rgb(0,0,0);stroke-width:6; fill:none" />

      <line x1="0" y1="200" x2="600" y2="200" style="stroke:rgb(0,0,0);stroke-width:5" />
      <line x1="0" y1="400" x2="600" y2="400" style="stroke:rgb(0,0,0);stroke-width:5" />

      <line x1="200" y1="0" x2="200" y2="600" style="stroke:rgb(0,0,0);stroke-width:5" />
      <line x1="400" y1="0" x2="400" y2="600" style="stroke:rgb(0,0,0);stroke-width:5" />

      <%= for x <- 0..9 do %>
      <line x1={x * 600/9} y1="0" x2={x*600/9} y2="600" style="stroke:rgb(0,0,0);stroke-width:1" />
      <% end %>

      <%= for y <- 0..9 do %>
      <line x1="0" y1={y * 600/9} x2="600" y2={y*600/9} style="stroke:rgb(0,0,0);stroke-width:1" />
      <% end %>
      </g>

      <g id="givens">
      <%= for x <- 0..8 do %>
      <%= for y <- 0..8 do %>
      <text font-size="45"
      fill="black"
      font-family="Verdana"
      text-anchor="middle"
      alignment-baseline="central"
      x={x*600/9+33}
      y={y*600/9+49}><%= assigns.sudoku |> Enum.at(x) |> Enum.at(y) %></text>
      <% end %>
      <% end %>
      </g>
    </svg>
    """
  end
end
