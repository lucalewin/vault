<script setup lang="ts">
defineProps<{
  title: string,
  to: string,
  icon: string
}>()
</script>

<template>
  <RouterLink
    :to="to"
    class="flex items-center p-2.5 px-4 rounded-lg group gap-2 hover:bg-neutral-800"
    active-class="text-amber-600 font-semibold bg-neutral-950/60"
  >
    <i :class="`w-5 h-5 pt-0.5 pi pi-${icon}`"></i>
    <span class="ms-2 text-sm">{{ title }}</span>
  </RouterLink>
</template>
