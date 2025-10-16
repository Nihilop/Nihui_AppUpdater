<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { Trash2, AlertTriangle } from 'lucide-vue-next';

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
  addonName: string;
  addonNiceName: string;
}

defineProps<Props>();
const emit = defineEmits<{
  'update:open': [value: boolean];
  confirm: [];
  cancel: [];
}>();

const isUninstalling = ref(false);

const handleConfirm = () => {
  isUninstalling.value = true;
  emit('confirm');
};
</script>

<template>
  <Dialog :open="open" @update:open="emit('update:open', $event)">
    <DialogContent class="max-w-md">
      <DialogHeader>
        <DialogTitle class="flex items-center gap-2">
          <AlertTriangle :size="20" class="text-orange-400" />
          {{ t('addons.uninstall.title') }}
        </DialogTitle>
        <DialogDescription>
          {{ t('addons.uninstall.description') }}
        </DialogDescription>
      </DialogHeader>

      <div class="space-y-3 py-4">
        <div class="flex items-center gap-3 p-4 bg-muted rounded-lg border border-orange-400/30">
          <Trash2 :size="24" class="text-orange-400 shrink-0" />
          <div class="flex-1 min-w-0">
            <p class="text-sm font-medium text-foreground">{{ addonNiceName }}</p>
            <p class="text-xs text-muted-foreground mt-1">{{ addonName }}</p>
          </div>
        </div>

        <p class="text-xs text-muted-foreground text-center px-4">
          {{ t('addons.uninstall.warning') }}
        </p>
      </div>

      <DialogFooter class="flex gap-2">
        <Button
          type="button"
          variant="outline"
          @click="emit('cancel')"
          :disabled="isUninstalling"
          class="flex-1"
        >
          {{ t('addons.uninstall.cancel') }}
        </Button>
        <Button
          type="button"
          variant="destructive"
          @click="handleConfirm"
          :disabled="isUninstalling"
          class="flex-1"
        >
          <Trash2 v-if="!isUninstalling" :size="16" class="mr-2" />
          {{ isUninstalling ? t('addons.uninstall.uninstalling') : t('addons.uninstall.confirm') }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
