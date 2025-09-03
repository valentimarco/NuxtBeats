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
    if (err || res.status === "error") {
      toast.add({ color: "error", title: "Error fetching playlists", description: res?.status === "error" ? res.error.toString() : err?.message })
      return
    }
    favorites.value = { id: res.data.find(p => p.id === "LM")?.id || "LM", songs: [] }
    console.log(res.data)
    playlists.value = res.data.filter(p => p.id !== "LM")
  }

  async function fetchSongsFromPlaylist(id: string, offset: number | null, limit: number | null) {
    const [err, res] = await tryCatch(commands.getSongsFromPlaylist(id, offset, limit))
    if (err || res.status === "error") {
      toast.add({ color: "error", title: "Error fetching songs", description: res?.status === "error" ? res.error.toString() : err?.message })
      return
    }

    console.dir(res)
    return res.data
  }

  return {
    playlists,
    favorites,
    fetchPlaylists,
    fetchSongsFromPlaylist
  }
}
