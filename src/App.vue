<script setup lang="ts">
import { computed, nextTick, ref, useTemplateRef } from "vue";
import { invoke } from "@tauri-apps/api/core";
import ImgList from "./components/img-list.vue";
import ExtractResult from "./components/extract-result.vue";
import {
  BinaryImage,
  FailResp,
  ImageResult,
  SelectedImage,
  SuccessResp,
} from "./models";

const selectImgs = useTemplateRef("selectImgs");

const allowExtract = computed(() => {
  return !loadingImgs.value && imgList.value.length > 0;
});
const loadingImgs = ref(false);
const extracting = ref(false);
const imgList = ref([] as SelectedImage[]);
const resultList = ref({ success: [], fail: [] } as ImageResult);

function handleSelectImgs() {
  selectImgs.value?.click();
}

async function handleInputImgs(ev: Event) {
  loadingImgs.value = true;
  let ele = ev.target as HTMLInputElement;

  for (const newImg of ele.files ?? []) {
    let bytes = new Uint8Array(await newImg.arrayBuffer());
    let hashed: string = await invoke("hash_file", { bytes: bytes });

    if (!imgList.value.some((img) => img.id === hashed)) {
      imgList.value.push(new SelectedImage(newImg, hashed));
    }
  }

  ele.value = "";
  loadingImgs.value = false;
}

function clearAll() {
  imgList.value = [];
}

function handleRemoveFile(file: SelectedImage) {
  imgList.value = imgList.value.filter((f) => f.id !== file.id);
}

async function handleExtract() {
  extracting.value = true;

  nextTick(async () => {
    const preImages = await Promise.all(
      imgList.value.map(async (img) => {
        const bytes = new Uint8Array(await img.file.arrayBuffer());
        return new BinaryImage(img.id, bytes);
      })
    );

    const coreNums = await invoke("core_nums");

    setTimeout(async function foo() {
      if (preImages.length === 0) {
        extracting.value = false;
        return;
      }

      let count = 0;
      let temp = [];
      while (preImages.length > 0) {
        temp.push(preImages.shift());
        count++;
        if (count === coreNums) {
          break;
        }
      }

      const resp: {
        success: {
          id: string;
          track_number: string;
          rect: {
            top_left: {
              x: number;
              y: number;
            };
            bottom_right: {
              x: number;
              y: number;
            };
          };
        }[];
        fail: string[];
      } = await invoke("extract", { images: temp });

      resultList.value.success.push(
        ...resp.success.map(
          (e) =>
            new SuccessResp(
              e.id,
              e.track_number,
              imgList.value.find((img) => img.id === e.id)!.file,
              {
                top: e.rect.top_left.y,
                right: e.rect.bottom_right.x,
                bottom: e.rect.bottom_right.y,
                left: e.rect.top_left.x,
              }
            )
        )
      );

      resultList.value.fail.push(
        ...resp.fail.map(
          (e) =>
            new FailResp(e, imgList.value.find((img) => img.id === e)!.file)
        )
      );

      setTimeout(foo, 0);
    }, 0);
  });
}

function handleTrigger(id: string) {
  let successTrigger = resultList.value.success.find((t) => t.id === id);
  if (successTrigger) {
    successTrigger.hide = !successTrigger?.hide;
  }

  let failTrigger = resultList.value.fail.find((t) => t.id === id);
  if (failTrigger) {
    failTrigger.hide = !failTrigger?.hide;
  }
}

function handleCleanSuccess() {
  resultList.value.success = [];
}

function handleCleanFail() {
  resultList.value.fail = [];
}
</script>

<template>
  <main class="flex p-4">
    <section
      class="flex flex-col border-r-2 px-4 min-h-ful w-2xl gap-2 relative"
    >
      <div>
        <button
          class="btn btn-primary"
          @click="handleSelectImgs"
          :disabled="loadingImgs"
        >
          {{ loadingImgs ? "加载图片中" : "选择图片" }}
        </button>
        <input
          type="file"
          ref="selectImgs"
          multiple
          accept="image/png, image/jpeg"
          hidden
          @change="handleInputImgs"
        />
      </div>
      <div class="flex flex-col gap-2">
        <div class="space-x-2">
          <button
            class="btn btn-accent"
            @click="handleExtract"
            :disabled="!allowExtract"
          >
            开始提取
          </button>
          <button class="btn btn-error" @click="clearAll">清除所有</button>
        </div>
        <div class="" v-show="imgList.length">
          选择了 {{ imgList.length }} 张图片
        </div>
        <ImgList :list="imgList" @remove="handleRemoveFile" />
      </div>

      <div
        class="absolute inset-0 bg-gray-400/50 flex flex-col gap-2 items-center justify-center"
        v-show="extracting"
      >
        <div class="bg-gray-50/70 rounded p-2">
          提取中
          <span class="loading loading-spinner text-accent"></span>
        </div>
      </div>
    </section>
    <section class="flex flex-col p-4 w-full h-full">
      <ExtractResult
        :image-result="resultList"
        @trigger="handleTrigger"
        @cleanSuccess="handleCleanSuccess"
        @cleanFail="handleCleanFail"
      />
    </section>
  </main>
</template>
