<script lang="ts" setup>

//(currentTime / songDuration) * 100
const musicProgress = ref(10)

const volume = ref(37)

</script>

<template>
    <div class="flex flex-row justify-between bg-zinc-800 border-t border-zinc-700 py-4 px-6 fixed bottom-0 left-0 right-0">
        <!-- Song Info -->
        <div class="flex items-center mb-4 md:mb-0">
            <UAvatar icon="i-lucide-music"
                class="w-12 h-12 bg-gradient-to-br from-purple-600 to-pink-500 rounded flex items-center justify-center mr-3" />
            <div>
                <h3 class="font-medium">Title</h3>
                <p class="text-sm text-gray-400">Artist</p>
            </div>

            <UButton icon="i-lucide-heart" variant="ghost" size="md" class="p-2 ml-4 rounded-full" />

            <!-- <button class="ml-4 p-2 rounded-full hover:bg-zinc-700">
                    <Heart class="h-5 w-5" :class="{ 'text-red-500 fill-red-500': isFavorite }"
                        @click="toggleFavorite" />
                </button> -->
        </div>

        <!-- Player Controls -->
        <div class="flex flex-col md:w-1/2 items-center">
            <div class="flex items-center space-x-4 mb-3">
                <UButton icon="i-lucide-skip-back" variant="ghost" size="md"
                    class="p-2 rounded-full hover:bg-zinc-700" />
                <UButton icon="i-lucide-play" variant="ghost" size="md"
                    class="p-3 rounded-full hover:bg-zinc-700 transition-colors" />
                <!-- <Pause v-if="isPlaying" class="h-6 w-6" />
                        <Play v-else class="h-6 w-6" /> -->
                <UButton icon="i-lucide-skip-forward" variant="ghost" size="md"
                    class="p-2 rounded-full hover:bg-zinc-700" />
            </div>

            <div class="w-full flex items-center space-x-3">
                <span class="text-xs text-gray-400">1:30</span>
                <div class="flex-1 h-1 bg-zinc-700 rounded-full overflow-hidden">
                    <UProgress v-model="musicProgress" :max="15" class="h-full bg-purple-500" @click="seekTo" />
                </div>
                <span class="text-xs text-gray-400">2:30</span>
            </div>
        </div>
        <!-- Volume Controls -->
        <div class="flex items-center mt-4 md:mt-0">
            <UButton icon="i-lucide-repeat" variant="ghost" size="md" class="p-2 rounded-full hover:bg-zinc-700 mr-2" />
            <UButton icon="i-lucide-shuffle" variant="ghost" size="md"
                class="p-2 rounded-full hover:bg-zinc-700 mr-2" />

            <div class="w-24 bg-zinc-700 rounded-full overflow-hidden">
                <USlider v-model="volume" size="md" />
            </div>
        </div>
    </div>

</template>