<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { X } from 'lucide-vue-next';
import { Button } from '@/components/ui/button';

const appWindow = getCurrentWindow();

const closeWindow = async () => {
  await appWindow.hide();
};
</script>

<template>
  <div
      data-tauri-drag-region
    class="sticky top-0 left-0 w-screen h-8 flex items-center justify-between px-3 select-none"
  >
    <div class="flex items-center gap-2">
      <span class="text-xs font-semibold text-foreground">Nihui Addon Updater</span>
    </div>

    <Button
      variant="ghost"
      size="icon"
      class="h-6 w-6 hover:bg-destructive/20 hover:text-destructive transition-colors"
      @click="closeWindow"
    >
      <X :size="14" />
    </Button>
  </div>
</template>