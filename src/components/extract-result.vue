<script setup lang="ts">
const { imageResult } = defineProps<{
  imageResult: ImageResult;
}>();

const emits = defineEmits<{
  (e: "trigger", id: string): void;
  (e: "cleanSuccess"): void;
  (e: "cleanFail"): void;
}>();

import { ImageResult } from "../models";
import { url } from "../utilities";
import { save } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";

function handleCopy(ev: MouseEvent, trackNumber: string) {
  navigator.clipboard.writeText(trackNumber);
  (ev.target as HTMLButtonElement).innerText = "√";
}

function handleTrigger(id: string) {
  emits("trigger", id);
}

function handleCleanSuccess() {
  emits("cleanSuccess");
}

function handleCleanFail() {
  emits("cleanFail");
}

async function handleExportExcel() {
  const path = await save({
    filters: [
      {
        name: "选择保存路径",
        extensions: ["xlsx"],
      },
    ],
  });

  const list = await Promise.all(
    imageResult.success.map(async (f) => {
      const bytes = new Uint8Array(await f.file.arrayBuffer());
      return {
        id: f.id,
        track_number: f.trackNumber,
        bytes,
      };
    })
  );

  await invoke("create_write_excel", { list, to: path });
}
</script>

<template>
  <div class="flex flex-col gap-4">
    <section class="flex flex-col">
      <div class="flex items-center gap-4">
        <div class="text-accent text-2xl">
          识别成功 {{ imageResult.success.length }} 张图片
        </div>
        <button
          class="btn btn-accent"
          @click="handleCleanSuccess"
          :disabled="imageResult.success.length === 0"
        >
          清除所有
        </button>
        <button
          class="btn"
          :disabled="imageResult.success.length === 0"
          @click="handleExportExcel"
        >
          导出到 Excel
        </button>
      </div>
      <ul v-for="img in imageResult.success" v-viewer>
        <li :key="img.id">
          <div class="flex items-center gap-2">
            <div class="flex items-center" v-show="!img.hide">
              <span class="pt-4 border-t w-28">
                <img :src="url(img.file)" :alt="img.trackNumber" />
              </span>
              <div class="grow text-3xl">
                {{ img.trackNumber }}
                <button
                  class="btn btn-lg btn-dash"
                  @click="(ev: MouseEvent) => handleCopy(ev, img.trackNumber)"
                >
                  复制
                </button>
              </div>
            </div>
            <div class="flex justify-end grow">
              <button class="btn" @click="handleTrigger(img.id)">
                {{ img.hide ? "显示" : "隐藏" }}
              </button>
            </div>
          </div>
        </li>
      </ul>
    </section>
    <section class="flex flex-col border-t-4">
      <div class="text-error text-2xl">
        无法识别 {{ imageResult.fail.length }} 张图片
        <button
          class="btn btn-accent"
          @click="handleCleanFail"
          :disabled="imageResult.fail.length === 0"
        >
          清除所有
        </button>
      </div>
      <ul v-for="img in imageResult.fail" v-viewer>
        <li :key="img.id">
          <div class="flex items-center gap-2">
            <span class="pt-4 border-t w-28" v-show="!img.hide">
              <img :src="url(img.file)" :alt="img.id" />
            </span>
            <div class="flex justify-end grow">
              <button class="btn" @click="handleTrigger(img.id)">
                {{ img.hide ? "显示" : "隐藏" }}
              </button>
            </div>
          </div>
        </li>
      </ul>
    </section>
  </div>
</template>
