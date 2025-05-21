export const useDataStore = defineStore("Data", () => {
  const playlists = ref<Record<string, any>[]>([
    {
      id: "playlist-1",
      name: "Playlist 1",
      description: "Description of Playlist 1",
      cover: "/images/playlist-cover-1.jpg",
    },
    {
      id: "playlist-2",
      name: "Playlist 2",
      description: "Description of Playlist 2",
      cover: "/images/playlist-cover-2.jpg",
    },
  ])

  return {
    playlists,
  }
})
