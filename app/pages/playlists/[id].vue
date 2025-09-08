<script setup lang="ts">
import type { TableColumn } from '@nuxt/ui'
import { UAvatar } from '#components'

definePageMeta({
  middleware: [async (to) => {
    const { playlists } = await usePlaylists()
    const id = (to.params as { id: string }).id
    if (!playlists.value.some(i => i.id === id))
      throw createError({ statusCode: 404, statusMessage: 'Playlist not found', fatal: true })
  }],
})

const { playlists } = await usePlaylists()
const route = useRoute()
const playlistId = (route.params as { id: string }).id
const playlist = computed(() => playlists.value.find(i => i.id === playlistId)!)

const { data: songs, status } = await useLazyAsyncData(`playlist-${playlistId}`, async () => {
  const [err, res] = await tryCatchTauri(commands.getSongsFromPlaylist(playlistId, null, null))
  if (err || !res) throw createError({ statusCode: 500, statusMessage: 'Failed to load songs' })
  return res
}, {
  default: () => [] as Song[],
})

const columns: TableColumn<Song>[] = [
  {
    accessorKey: 'id',
    header: '#',
    cell: ({ row }) => row.index + 1,
    meta: { class: { th: 'w-0' } },
  },
  {
    accessorKey: 'name',
    header: 'Name'
  },
  {
    accessorKey: 'album',
    header: 'Album',
    cell: ({ row }) => row.original.album.name || '-'
  },
  {
    accessorKey: 'time',
    header: 'Duration',
    cell: ({ row }) => Number(row.original.time)
  }
]

</script>

<template>
  <div class="grow">
    <div class="flex gap-2 md:gap-4">
      <NuxtImg height="192" width="192" class="size-48 rounded-md" :alt="playlist.name" quality="100"
        :src="playlist.cover[0]" />
      <div>
        <h2 class="font-bold text-lg text-center">
          {{ playlist.name }}
        </h2>
        <h3>{{ playlist.tracks }}</h3>
      </div>
    </div>
    <UTable :data="songs" :columns :loading="status === 'pending'" sticky />
  </div>
</template>
