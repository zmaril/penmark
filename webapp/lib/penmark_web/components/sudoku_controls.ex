defmodule PenmarkWeb.SudokuControls do
  # If you generated an app with mix phx.new --live,
  # the line below would be: use MyAppWeb, :live_component
  use Phoenix.LiveComponent
	alias Phoenix.LiveView.JS

  def controls(assigns) do
    ~H"""
    <div class="relative rounded-t-xl overflow-hidden bg-gradient-to-r from-blue-50 to-blue-100 bg-white w-full">
    <div class="grid grid-cols-3 gap-4">
      <%= for n <- 1..9 do %>
			<%= if assigns.selected_number == n do %>
    	<div phx-click={JS.push("select_number", value: %{number: n})}
			class="h-12 rounded-md flex items-center justify-center text-white text-2xl font-extrabold bg-blue-800">
			<%= n %>
			</div>
			<% else %>
    	<div phx-click={JS.push("select_number", value: %{number: n})}
			class="h-12 rounded-md flex items-center justify-center text-white text-2xl font-extrabold bg-blue-500">
			<%= n %>
			</div>
			<% end %>
      <% end %>
    </div>
    </div>
    """
  end
end
