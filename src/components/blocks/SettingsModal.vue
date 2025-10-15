<script setup lang="ts">
import { ref } from 'vue';
import { Search, Check } from 'lucide-vue-next';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { Separator } from '@/components/ui/separator';

interface Props {
  open: boolean;
  wowPath: string | null;
  autoScanResults: string[];
  isScanning: boolean;
  launchOnStartup: boolean;
}

defineProps<Props>();
const emit = defineEmits<{
  'update:open': [value: boolean];
  selectPath: [path: string];
  validateManualPath: [path: string];
  autoScan: [];
  'update:launchOnStartup': [value: boolean];
}>();

const manualPath = ref('');

const handleValidate = () => {
  if (manualPath.value) {
    emit('validateManualPath', manualPath.value);
    manualPath.value = '';
  }
};
</script>

<template>
  <Dialog :open="open" @update:open="emit('update:open', $event)">
    <DialogContent class="w-screen h-screen !max-w-screen rounded-none p-0 flex flex-col gap-0">
      <DialogHeader class="shrink-0 px-6 py-4 border-b border-border">
        <DialogTitle>WoW Installation Settings</DialogTitle>
        <DialogDescription>
          Configure your World of Warcraft installation path
        </DialogDescription>
      </DialogHeader>

      <div class="flex-1 overflow-y-auto px-6 py-4 space-y-4">
        <!-- Auto-scan Results -->
        <div v-if="autoScanResults.length > 0" class="space-y-2">
          <Label>Found installations:</Label>
          <div class="space-y-2 max-h-48 overflow-y-auto">
            <button
              v-for="path in autoScanResults"
              :key="path"
              @click="emit('selectPath', path)"
              class="w-full p-3 bg-muted hover:bg-accent rounded border border-border text-left text-sm transition-colors flex items-center justify-between group"
            >
              <span class="truncate">{{ path }}</span>
              <Check :size="16" class="text-green-400 opacity-0 group-hover:opacity-100 transition-opacity shrink-0 ml-2" />
            </button>
          </div>
        </div>

        <!-- Manual Path Input -->
        <div class="space-y-2">
          <Label>Or enter path manually:</Label>
          <div class="flex gap-2">
            <Input
              v-model="manualPath"
              type="text"
              placeholder="E:\Battle.net\World of Warcraft"
              class="flex-1"
              @keyup.enter="handleValidate"
            />
            <Button
              type="button"
              @click="handleValidate"
            >
              Validate
            </Button>
          </div>
          <p class="text-xs text-muted-foreground">Example: E:\Battle.net\World of Warcraft</p>
        </div>

        <!-- Scan Button -->
        <Button
          type="button"
          @click="emit('autoScan')"
          :disabled="isScanning"
          variant="default"
          class="w-full"
        >
          <Search :size="16" class="mr-2" :class="{ 'animate-spin': isScanning }" />
          {{ isScanning ? 'Scanning...' : 'Scan Again' }}
        </Button>

        <Separator />

        <!-- Launch on Startup Toggle -->
        <div class="flex items-center justify-between">
          <div class="space-y-0.5">
            <Label class="text-base">Launch on Windows startup</Label>
            <p class="text-sm text-muted-foreground">
              Automatically start Nihui Addon Updater when Windows boots
            </p>
          </div>
          <Switch
              :model-value="launchOnStartup"
            @update:checked="emit('update:launchOnStartup', $event)"
          />
        </div>
      </div>

      <DialogFooter class="shrink-0 px-6 py-4 border-t border-border">
        <Button
          type="button"
          variant="outline"
          @click="emit('update:open', false)"
          class="w-full"
        >
          Close
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
