<script setup lang="ts">
import { ref } from 'vue';
import SidebarItem from '../components/SidebarItem.vue';

const isSidebarOpen = ref(false);
const toggleSidebar = () => {
  isSidebarOpen.value = !isSidebarOpen.value;
};
const closeSidebar = () => {
  isSidebarOpen.value = false;
};
</script>

<template>
  <nav class="fixed top-0 w-full text-white bg-neutral-800 border-b border-neutral-600 sm:ml-64">
    <div class="p-3 sm:p-4 flex items-center">
      <span class="ml-2 w-8 h-8 text-xl text-neutral-400 hover:bg-neutral-600 rounded p-1 sm:hidden" @click="toggleSidebar">
        <i class="pi pi-bars w-4 h-4 pt-0.5 pl-0.5"></i>
      </span>
      <h1 class="text-xl sm:text-3xl font-semibold px-2">Home</h1>
    </div>
  </nav>

  <aside 
    :class="{'translate-x-0': isSidebarOpen, '-translate-x-full': !isSidebarOpen}"  
    class="fixed top-0 left-0 w-64 border-r border-neutral-600 h-dvh transition-transform sm:translate-x-0 z-50"
  >
    <div class="h-dvh p-4 flex flex-col justify-between bg-neutral-800">
      <span>
        <!-- Header + Sidebar-Cross -->
        <div class="flex items-center mt-2 sm:text-center sm:block">
          <span class="mx-2.5 w-8 h-8 text-xl text-neutral-400 hover:bg-neutral-600 rounded p-1 sm:hidden">
            <i class="pi pi-times w-4 h-4 pt-0.5 pl-0.5" @click="toggleSidebar"></i>
          </span>
          <RouterLink class="text-lg sm:text-3xl text-center font-bold text-white" to="/">Ferrum Vault</RouterLink>
        </div>
        <!-- Top Sidebar Items -->
        <div class="space-y-1 mt-8 overflow-y-auto" @click="closeSidebar">
          <slot name="sidebar_items"></slot>
        </div>
      </span>
      <!-- Bottom Sidebar Items -->
      <div class="space-y-1" @click="toggleSidebar">
        <slot name="sidebar_items_bottom"></slot>
      </div>
    </div>
  </aside>
  <!-- fills the rest of the page when the sidebar is open (in mobile mode) and closes the sidebar when clicked -->
  <div class="h-dvh ml-64 w-full fixed" :class="{'hidden': !isSidebarOpen}" @click="toggleSidebar"></div>

  <main>
    <div class="sm:ml-64 pt-17 min-h-screen bg-neutral-900">
      <div class="p-3">
        <slot></slot>
      </div>
    </div>
  </main>
</template>