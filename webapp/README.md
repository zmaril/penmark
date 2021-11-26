# Penmark

Everybody loves sudoku.

# Development 

```sh
git clone git@github.com/zmaril/penmark
cd webapp
mix deps.get
mix ecto.setup 
mix phx.server 
iex -S mix phx.server 
```

Visit [`localhost:4000`](http://localhost:4000) next.

# Deployment 

[Taken from fly docs](https://fly.io/docs/getting-started/elixir/)
```sh
fly deploy
fly status
fly logs 
```

# Debug
``` sh
# Open a shell on fly
fly ssh establish
fly ssh issue
ssh-add ~/.ssh/id_fly
fly ssh console
app/entry remote
# iex> 
// TODO set up livedashboard and livebook
```


# Useful/used links 
* Tailwind setup with elixir ([link](https://pragmaticstudio.com/tutorials/adding-tailwind-css-to-phoenix))
* tictac elixir app on fly ([link](https://github.com/fly-apps/tictac))
# TODO's 
http -> https, namecheap? 
add this to github 
make it do something useful 
buy domains and set up redirects to load in sudokus from other sites 
master branch protection 
deploy from push to master on github 
favicon 
social descriptions 
generate images of things 
generate gifs 
solve sudokous 
setting, editing puzzles 
playing,solving, editing solutions 