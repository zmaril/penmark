defmodule PenmarkWeb.SudokuLive do
  use Phoenix.LiveView

  def render(assigns) do
    ~H"""
    Current temperature: <%= @temperature %>
    """
  end
  def mount(_params, %{}, socket) do
    {:ok, assign(socket, :temperature, 10)}
  end
end
