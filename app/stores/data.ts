export const useDataStore = defineStore("Data", () => {
  const playlists = ref<Playlists[]>([])

  const fetchPlaylists = async () => {
    const result = await commands.getPlaylists()
    if (result.status === "error") throw new Error(`${result.error}`)
    playlists.value = result.data
  }
  return {
    playlists,
    fetchPlaylists,
  }
})
