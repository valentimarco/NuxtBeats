export async function usePlaylists() {
  const playlists = useState<Playlist[]>("allPlaylists", () => [])
  const favorites = useState<Favorites>("favoriteSongs", () => ({ id: "LM", songs: [] }))

  const toast = useToast()

  await callOnce(async () => {
    console.info("Fetching playlists...")
    await fetchPlaylists()
  })

  async function fetchPlaylists() {
    const [err, res] = await tryCatch(commands.getAllPlaylists())
    if (err) {
      toast.add({ color: "error", title: "Error fetching playlists", description: err.message })
      return
    }
    favorites.value = { id: res.find(p => p.id === "LM")?.id || "LM", songs: [] }
    playlists.value = res.filter(p => p.id !== "LM")
  }

  return {
    playlists,
    favorites,
    fetchPlaylists
  }
}
