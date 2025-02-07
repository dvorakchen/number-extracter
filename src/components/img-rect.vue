<script setup lang="ts">
const { rect } = defineProps<{
  file: File;
  trackNumber: string;
  rect: Rectangle;
}>();

import { Rectangle } from "../models";
import { url } from "../utilities";
import { onMounted, useTemplateRef } from "vue";

const img = useTemplateRef("img");
const can = useTemplateRef("rectCan");

onMounted(drawRectangle);

function drawRectangle() {
  const ctx = can.value!.getContext("2d")!;

  img.value!.onload = () => {
    console.log("img loaded");
    can.value!.width = img.value!.naturalWidth;
    can.value!.height = img.value!.naturalHeight;
    ctx.drawImage(img.value!, 0, 0, can.value!.width, can.value!.height);

    ctx.strokeStyle = "red";
    ctx.lineWidth = 3;
    ctx.strokeRect(
      rect.left,
      rect.top,
      rect.right - rect.left,
      rect.bottom - rect.top
    );

    img.value!.onload = null;
    img.value!.src = can.value!.toDataURL("image/jpeg");
  };
}
</script>

<template>
  <span class="pt-4 border-t w-28">
    <img ref="img" :src="url(file)" :alt="trackNumber" />
  </span>
  <canvas ref="rectCan" class="hidden"></canvas>
</template>
