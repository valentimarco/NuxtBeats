export async function usePlaylists() {
  const playlists = useState<Playlists[]>(() => [])

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
    playlists.value = res.data
  }

  return {
    playlists,
    fetchPlaylists,
  }
}
