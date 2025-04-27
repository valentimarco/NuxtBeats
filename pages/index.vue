<script lang="ts" setup>
const songs = ref([
    { title: 'Midnight City', artist: 'M83', duration: '4:03', favorite: false },
    { title: 'Blinding Lights', artist: 'The Weeknd', duration: '3:22', favorite: true },
    { title: 'Dreams', artist: 'Fleetwood Mac', duration: '4:17', favorite: false },
    { title: 'Don\'t Start Now', artist: 'Dua Lipa', duration: '3:19', favorite: false },
    { title: 'Redbone', artist: 'Childish Gambino', duration: '5:27', favorite: true },
    { title: 'Feel Good Inc', artist: 'Gorillaz', duration: '3:42', favorite: false },
    { title: 'Levitating', artist: 'Dua Lipa', duration: '3:23', favorite: false },
    { title: 'Uptown Funk', artist: 'Mark Ronson ft. Bruno Mars', duration: '4:30', favorite: false },
]);
const currentSongIndex = ref(0);
const currentSong = computed(() => songs.value[currentSongIndex.value]);
</script>


<template>
    <main class="flex-1 p-6 pb-32">
        <div class="flex flex-col gap-8">
            <!-- Playlist Section -->
            <div class="w-full bg-zinc-800 rounded-xl p-6 shadow-lg">
                <div class="flex items-center justify-between mb-6 text-center">
                    <h2 class="text-2xl font-bold">Your Playlist</h2>
                    <!-- <div class="flex space-x-2">
                        <UButton icon="i-lucide-list-filter" class="p-2 rounded-full hover:bg-zinc-700" size="md" variant="ghost"/>
                        <UButton icon="i-lucide-plus" class="p-2 rounded-full hover:bg-zinc-700" size="md" variant="ghost"/>
                    </div> -->
                </div>

                <!-- Playlist Items -->
                <div class="space-y-2">
                    <div v-for="(song, index) in songs" :key="index"
                        class="flex items-center p-3 rounded-lg hover:bg-zinc-700 cursor-pointer transition-colors"
                        :class="{ 'bg-zinc-700': currentSongIndex === index }" @click="playSong(index)">
                        <div class="w-10 text-center">
                            <span v-if="currentSongIndex !== index">{{ index + 1 }}</span>
                            <UIcon v-else name="i-lucide-play" class="h-5 w-5 text-purple-500 mx-auto"/>
                        </div>
                        <div class="flex-1 ml-4">
                            <h3 class="font-medium">{{ song.title }}</h3>
                            <p class="text-sm text-gray-400">{{ song.artist }}</p>
                        </div>
                        <div class="text-gray-400 text-sm">{{ song.duration }}</div>
                    </div>
                </div>
            </div>
        </div>
    </main>
</template>
