defmodule Penmark.Repo do
  use Ecto.Repo,
    otp_app: :penmark,
    adapter: Ecto.Adapters.Postgres
end
