<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { Search, Check } from 'lucide-vue-next';

const { t } = useI18n();
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { availableLocales } from '@/locales';

interface Props {
  open: boolean;
  wowPath: string | null;
  autoScanResults: string[];
  isScanning: boolean;
  launchOnStartup: boolean;
  minimizeOnStartup: boolean;
  language: string;
}

defineProps<Props>();
const emit = defineEmits<{
  'update:open': [value: boolean];
  selectPath: [path: string];
  validateManualPath: [path: string];
  autoScan: [];
  'update:launchOnStartup': [value: boolean];
  'update:minimizeOnStartup': [value: boolean];
  'update:language': [value: string];
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
        <DialogTitle>{{ t('settings.title') }}</DialogTitle>
        <DialogDescription>
          {{ t('settings.description') }}
        </DialogDescription>
      </DialogHeader>

      <div class="flex-1 overflow-y-auto px-6 py-4 space-y-4">
        <!-- Auto-scan Results -->
        <div v-if="autoScanResults.length > 0" class="space-y-2">
          <Label>{{ t('settings.foundInstallations') }}</Label>
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
          <Label>{{ t('settings.manualPath') }}</Label>
          <div class="flex gap-2">
            <Input
              v-model="manualPath"
              type="text"
              :placeholder="t('settings.pathPlaceholder')"
              class="flex-1"
              @keyup.enter="handleValidate"
            />
            <Button
              type="button"
              @click="handleValidate"
            >
              {{ t('settings.validate') }}
            </Button>
          </div>
          <p class="text-xs text-muted-foreground">{{ t('settings.pathExample') }}</p>
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
          {{ isScanning ? t('settings.scanning') : t('settings.scanAgain') }}
        </Button>

        <Separator />

        <!-- Launch on Startup Toggle -->
        <div class="flex items-center justify-between">
          <div class="space-y-0.5">
            <Label class="text-base">{{ t('settings.launchOnStartup') }}</Label>
            <p class="text-sm text-muted-foreground">
              {{ t('settings.launchOnStartupDesc') }}
            </p>
          </div>
          <Switch
            :model-value="launchOnStartup"
            @update:modelValue="emit('update:launchOnStartup', $event)"
          />
        </div>

        <!-- Minimize on Startup Toggle -->
        <div class="flex items-center justify-between">
          <div class="space-y-0.5">
            <Label class="text-base">{{ t('settings.minimizeOnStartup') }}</Label>
            <p class="text-sm text-muted-foreground">
              {{ t('settings.minimizeOnStartupDesc') }}
            </p>
          </div>
          <Switch
            :model-value="minimizeOnStartup"
            @update:modelValue="emit('update:minimizeOnStartup', $event)"
          />
        </div>

        <!-- Language Select -->
        <div class="space-y-2">
          <Label class="text-base">{{ t('settings.language') }}</Label>
          <Select :model-value="language" @update:modelValue="emit('update:language', $event)">
            <SelectTrigger>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem v-for="locale in availableLocales" :key="locale.code" :value="locale.code">
                {{ locale.name }}
              </SelectItem>
            </SelectContent>
          </Select>
          <p class="text-sm text-muted-foreground">
            {{ t('settings.languageDesc') }}
          </p>
        </div>
      </div>

      <DialogFooter class="shrink-0 px-6 py-4 border-t border-border">
        <Button
          type="button"
          variant="outline"
          @click="emit('update:open', false)"
          class="w-full"
        >
          {{ t('settings.close') }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
