<script setup lang="ts">
defineProps<{
  title: string,
  to: string,
  icon: string
}>()
</script>

<template>
  <!-- <RouterLink
    :to="to"
    class="flex items-center p-2 px-4 rounded-lg hover:bg-neutral-700 group gap-2"
    active-class="text-green-500 font-semibold bg-green-500/20 hover:bg-green-500/50"
  >
    <svg class="w-5 h-5 transition duration-75 dark:group-hover:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 22 21">
       <path d="M16.975 11H10V4.025a1 1 0 0 0-1.066-.998 8.5 8.5 0 1 0 9.039 9.039.999.999 0 0 0-1-1.066h.002Z"/>
       <path d="M12.5 0c-.157 0-.311.01-.565.027A1 1 0 0 0 11 1.02V10h8.975a1 1 0 0 0 1-.935c.013-.188.028-.374.028-.565A8.51 8.51 0 0 0 12.5 0Z"/>
    </svg>
    <span class="ms-3">{{ title }}</span>
  </RouterLink> -->
  <RouterLink
    :to="to"
    class="flex items-center p-2.5 px-4 rounded-lg group gap-2 hover:bg-neutral-700"
    active-class="text-green-500 font-semibold bg-green-500/13 hover:bg-green-500/50"
  >
    <i :class="`w-5 h-5 pt-0.5 pi pi-${icon}`"></i>
    <span class="ms-2 text-sm">{{ title }}</span>
  </RouterLink>



  <!-- <RouterLink :to="to" activeClass="bg-orange-500 hover:bg-orange-400">
    <div class="flex items-center p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700 group">

      <span class="ms-3">{{ title }}</span>
    </div>
  </RouterLink> -->
        <!-- <svg class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 22 21">
         <path d="M16.975 11H10V4.025a1 1 0 0 0-1.066-.998 8.5 8.5 0 1 0 9.039 9.039.999.999 0 0 0-1-1.066h.002Z"/>
         <path d="M12.5 0c-.157 0-.311.01-.565.027A1 1 0 0 0 11 1.02V10h8.975a1 1 0 0 0 1-.935c.013-.188.028-.374.028-.565A8.51 8.51 0 0 0 12.5 0Z"/>
      </svg> -->
</template>