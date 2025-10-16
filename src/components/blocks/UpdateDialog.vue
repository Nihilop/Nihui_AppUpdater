<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { Download } from 'lucide-vue-next';

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

interface Props {
  open: boolean;
  currentVersion: string;
  newVersion: string;
}

defineProps<Props>();
const emit = defineEmits<{
  'update:open': [value: boolean];
  downloadAndInstall: [];
  remindLater: [];
}>();

const isInstalling = ref(false);

const handleInstall = () => {
  isInstalling.value = true;
  emit('downloadAndInstall');
};
</script>

<template>
  <Dialog :open="open" @update:open="emit('update:open', $event)">
    <DialogContent class="max-w-md">
      <DialogHeader>
        <DialogTitle class="flex items-center gap-2">
          <Download :size="20" class="text-blue-400" />
          {{ t('updater.updateAvailable') }}
        </DialogTitle>
        <DialogDescription>
          {{ t('updater.updateAvailableDesc') }}
        </DialogDescription>
      </DialogHeader>

      <div class="space-y-3 py-4">
        <div class="flex justify-between items-center p-3 bg-muted rounded-lg">
          <span class="text-sm text-muted-foreground">{{ t('updater.currentVersion') }}</span>
          <span class="text-sm font-medium">{{ currentVersion }}</span>
        </div>
        <div class="flex justify-between items-center p-3 bg-muted rounded-lg">
          <span class="text-sm text-muted-foreground">{{ t('updater.newVersion') }}</span>
          <span class="text-sm font-medium text-green-400">{{ newVersion }}</span>
        </div>
      </div>

      <DialogFooter class="flex gap-2">
        <Button
          type="button"
          variant="outline"
          @click="emit('remindLater')"
          :disabled="isInstalling"
          class="flex-1"
        >
          {{ t('updater.remindLater') }}
        </Button>
        <Button
          type="button"
          @click="handleInstall"
          :disabled="isInstalling"
          class="flex-1"
        >
          <Download v-if="!isInstalling" :size="16" class="mr-2" />
          {{ isInstalling ? t('updater.installing') : t('updater.downloadAndInstall') }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
