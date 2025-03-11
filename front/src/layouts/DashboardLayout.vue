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
  <nav class="fixed top-0 z-50 w-full bg-white border-b border-gray-200 dark:bg-gray-800 dark:border-gray-700">
    <div class="px-3 py-3 lg:px-5 lg:pl-3">
      <div class="flex items-center justify-between">
        <div class="flex items-center justify-start rtl:justify-end">
          <button @click="toggleSidebar" data-drawer-target="logo-sidebar" data-drawer-toggle="logo-sidebar" aria-controls="logo-sidebar" type="button" class="inline-flex items-center p-2 text-sm text-gray-500 rounded-lg sm:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600">
            <span class="sr-only">Open sidebar</span>
            <svg class="w-6 h-6" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
              <path clip-rule="evenodd" fill-rule="evenodd" d="M2 4.75A.75.75 0 012.75 4h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 4.75zm0 10.5a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5h-7.5a.75.75 0 01-.75-.75zM2 10a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 10z"></path>
            </svg>
          </button>
          <a href="/" class="flex ms-2 md:me-24">
            <img src="https://cdn-icons-png.flaticon.com/512/10806/10806945.png" class="h-8 me-3 hidden sm:block" alt="FlowBite Logo" />
            <span class="self-center text-xl font-semibold sm:text-2xl whitespace-nowrap dark:text-white">Ferrum Vault</span>
          </a>
        </div>
        <div class="flex items-center">
          <div class="flex items-center ms-3">
            <div>
              <button type="button" class="flex text-sm bg-gray-800 rounded-full focus:ring-4 focus:ring-gray-300 dark:focus:ring-gray-600" aria-expanded="false" data-dropdown-toggle="dropdown-user">
                <span class="sr-only">Open user menu</span>
                <img class="w-8 h-8 rounded-full" src="https://lucalewin.dev/_next/image?url=%2Flogo.png&w=64&q=75" alt="user photo">
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </nav>
  <div class="bg-gray-900 text-white">
    <aside :class="{'translate-x-0': isSidebarOpen, '-translate-x-full': !isSidebarOpen}" id="logo-sidebar" class="fixed top-0 left-0 z-40 w-64 h-screen pt-20 transition-transform bg-white border-r border-gray-200 sm:translate-x-0 dark:bg-gray-900 dark:border-gray-700" aria-label="Sidebar">
      <div class="h-full px-3 pb-4 overflow-y-auto bg-white dark:bg-gray-900">
        <!-- the click method only works good enough, if there is no gap between the sidebar items -->
        <div class="space-y-2 font-medium" @click="closeSidebar"> 
          <slot name="sidebar_items"></slot>
          <!-- <SidebarItem title="Home" to="/" @click="closeSidebar" />
          <SidebarItem title="Passwords" to="/passwords" @click="closeSidebar" />
          <SidebarItem title="Authenticator" to="/authenticator" @click="closeSidebar" /> -->
        </div>
      </div>
    </aside>
    <div class="sm:ml-64 pt-14 min-h-screen">
      <main class="p-3">
        <slot></slot>
      </main>
    </div>
  </div>
</template>