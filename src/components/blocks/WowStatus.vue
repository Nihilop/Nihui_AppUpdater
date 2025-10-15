green-400<script setup lang="ts">
import { computed } from 'vue';
import { HardDrive, Settings } from 'lucide-vue-next';
import { Button } from '@/components/ui/button';

interface Props {
  wowPath: string | null;
  isScanning: boolean;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  openSettings: [];
}>();

const statusColor = computed(() => {
  if (props.isScanning) return 'bg-orange-400';
  if (props.wowPath) return 'bg-green-400';
  return 'bg-red-400';
});

const statusText = computed(() => {
  if (props.isScanning) return 'Scanning...';
  if (props.wowPath) return 'Connected';
  return 'Not configured';
});
</script>

<template>
  <div class="flex items-center justify-between px-4 py-3 rounded-lg">
    <div class="flex items-center gap-3">
      <div class="relative">
        <HardDrive :size="24" class="text-muted-foreground" />
        <div
          :class="statusColor"
          class="absolute -bottom-1 -right-1 w-3 h-3 rounded-full border-2 border-card"
        />
      </div>
      <div>
        <p class="text-sm font-medium text-card-foreground">WoW Installation</p>
        <p class="text-xs text-muted-foreground">{{ statusText }}</p>
      </div>
    </div>

    <Button
      variant="ghost"
      size="icon"
      class="h-8 w-8"
      @click="emit('openSettings')"
    >
      <Settings :size="18" />
    </Button>
  </div>
</template>
