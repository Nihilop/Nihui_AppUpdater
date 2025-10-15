<script setup lang="ts">
import { computed } from 'vue';
import { Download, RefreshCw, ArrowUp, Settings2 } from 'lucide-vue-next';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import type { AddonStatus } from '@/types';

interface Props {
  addon: AddonStatus;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  install: [];
  update: [];
  check: [];
  configure: [];
}>();

const statusBadgeClass = computed(() => {
  if (props.addon.status === 'up-to-date') return 'bg-green-400 border-green-400';
  if (props.addon.status === 'update-available') return 'bg-orange-400 border-orange-400';
  if (!props.addon.is_installed) return 'bg-gray-500 border-gray-500';
  return 'bg-gray-500 border-gray-500';
});
</script>

<template>
  <div class="group flex items-center justify-between p-3 bg-card rounded-lg border border-border hover:border-accent transition-colors">
    <div class="flex items-center gap-3 flex-1 min-w-0">
      <Badge :class="statusBadgeClass" class="w-2 h-2 p-0 rounded-full shrink-0" />

      <div class="flex-1 min-w-0">
        <p class="text-sm font-medium text-card-foreground truncate">
          {{ addon.definition.nice_name }}
        </p>
      </div>
    </div>

    <div class="flex items-center gap-1 ml-3">
      <!-- Check button (always visible) -->
      <Button
        variant="ghost"
        size="icon"
        class="h-8 w-8 text-primary opacity-0 group-hover:opacity-100 transition-all duration-500"
        :disabled="addon.status === 'checking'"
        @click="emit('check')"
      >
        <RefreshCw :size="16" :class="{ 'animate-spin': addon.status === 'checking' }" />
      </Button>

      <!-- Install button (if not installed) -->
      <Button
        v-if="!addon.is_installed"
        variant="ghost"
        size="icon"
        class="h-8 w-8 text-green-400 hover:text-green-300"
        @click="emit('install')"
      >
        <Download :size="16" />
      </Button>

      <!-- Update button (if update available) -->
      <Button
        v-if="addon.is_installed && addon.update_available"
        variant="ghost"
        size="icon"
        class="h-8 w-8 text-orange-400 hover:text-orange-300"
        @click="emit('update')"
      >
        <ArrowUp :size="16" />
      </Button>

      <!-- Configure button -->
      <Button
        variant="ghost"
        size="icon"
        class="h-8 w-8 text-muted-foreground hover:text-foreground"
        @click="emit('configure')"
      >
        <Settings2 :size="16" />
      </Button>
    </div>
  </div>
</template>
