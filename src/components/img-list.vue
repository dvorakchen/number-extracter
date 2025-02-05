<script setup lang="ts">
defineProps<{
    list: SelectedImage[]
}>();

import { url } from '../utilities';

const emit = defineEmits<{
    (e: 'remove', file: SelectedImage): void
}>();

import 'viewerjs/dist/viewer.css'
import { SelectedImage } from '../models';

function removeImg(img: SelectedImage) {
    emit('remove', img);
}

</script>

<template>

    <div class="">
        <ul class="flex flex-wrap gap-2" v-viewer>
            <li class="flex" v-for="img in list" :key="img.file.name">
                <span class="w-24 border rounded cursor-pointer relative">
                    <div class="absolute top-0 right-0 badge badge-accent badge-sm" @click="removeImg(img)">X</div>
                    <img :src="url(img.file)" :alt="img.file.name" name="previewImg">
                </span>
            </li>
        </ul>

    </div>

</template>