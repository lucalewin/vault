<script setup lang="ts">
defineProps({
  show: Boolean
})
</script>

<template>
  <Transition name="modal">
    <div v-if="show" class="fixed z-100 inset-0 bg-black/50 flex items-center justify-center">
      <div class="bg-neutral-800 p-2 sm:p-4 rounded shadow-lg max-w-96 w-full">
        <div>
          <slot name="header"></slot>
        </div>

        <div class="my-2">
          <slot name="body"></slot>
        </div>

        <div class="float-right">
          <slot name="footer">
            <button
              class="w-full px-4 py-2 font-bold text-white bg-green-700/90 rounded hover:bg-green-700 focus:bg-green-700"
              @click="$emit('close')"
            >Okay</button>
          </slot>
        </div>
      </div>
    </div>
  </Transition>
</template>