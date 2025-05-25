// Temp fix
interface Favorites {
  id: string
  songs: Record<string, any>[]
}

export async function usePlaylists() {
  const playlists = useState<Playlist[]>("allPlaylists", () => [])
  const favorites = useState<Favorites>("favoriteSongs", () => ({ id: "LM", songs: [] }))

  const toast = useToast()

  await callOnce(async () => {
    console.info("Fetching playlists...")
    await fetchPlaylists()
  })

  async function fetchPlaylists() {
    const [err, res] = await tryCatch(commands.getPlaylists())
    if (err || res.status === "error") {
      toast.add({ color: "error", title: "Error fetching playlists", description: res?.status === "error" ? res.error.toString() : err?.message })
      return
    }
    favorites.value = { id: res.data.find(p => p.id === "LM")?.id || "LM", songs: [] }
    playlists.value = res.data.filter(p => p.id !== "LM")
  }

  return {
    playlists,
    favorites,
    fetchPlaylists,
  }
}
