<template>
  <v-app>
    <v-app-bar app>
      <template #append>
        <div class="d-flex">
          <div
            v-if="
              currentConfigUid && (
                currentFFStatus?.currentState === 'Running' ||
                currentFFStatus?.currentState === 'Waiting')
            "
            class="d-inline-flex justify-center align-center"
          >
            <v-tooltip
              text="
            RAM"
              location="bottom"
            >
              <template #activator="{ props }">
                <div
                  v-bind="props"
                  style="display: inline-block; min-width: 85px"
                >
                  {{
                    utils.formatBytes(Number(currentFFStatus?.memoryUsage))
                  }}
                </div>
              </template>
            </v-tooltip>

            <v-tooltip
              text="CPU Usage"
              location="bottom"
            >
              <template #activator="{ props }">
                <div
                  v-bind="props"
                  style="display: inline-block; min-width: 85px"
                >
                  - {{ (currentFFStatus?.cpuUsage ?? 0) / 100 }}%
                </div>
              </template>
            </v-tooltip>
          </div>

          <div
            v-if="currentConfigUid"
            class="d-inline-flex justify-center align-center"
          >
            <ff-status-icon
              :config-uid="currentConfigUid ?? ''"
            />
          </div>

          <div
            class="d-inline-flex justify-center align-center ma-2"
            style="transform: translateY(-2px);"
          >
            <Icon
              name="cust-icon:ses"
              filled
            />
          </div>

          <div
            class="d-inline-flex justify-center align-center"
          >
            <v-btn @click="toggleTheme">
              <v-icon>
                {{
                  theme.global.name.value === "dark"
                    ? "mdi-white-balance-sunny"
                    : "mdi-weather-night"
                }}
              </v-icon>
            </v-btn>
          </div>
        </div>
      </template>

      <v-btn
        v-if="currentRoute.name !== 'index'"
        icon
        @click.prevent="goToPrev"
      >
        <v-icon>
          {{
            currentRoute.name === "index" ? "mdi-home" : "mdi-arrow-left"
          }}
        </v-icon>
      </v-btn>

      <v-app-bar-title>
        {{ title }}
        <span v-if="ffconfig">- {{ ffconfig.name }}</span>
      </v-app-bar-title>
    </v-app-bar>

    <v-main>
      <v-container fluid>
        <slot />
      </v-container>
    </v-main>
  </v-app>
</template>
<script setup lang="ts">
// Theme settings
const theme = useTheme();

function toggleTheme() {
  theme.global.name.value = getTheme() == "dark" ? "light" : "dark";
  localStorage.setItem("theme", theme.global.name.value);
}

setTheme();

function setTheme() {
  theme.global.name.value = getTheme();
}

function getTheme() {
  const localStorageTheme = localStorage.getItem("theme");
  const systemSettingDark = window.matchMedia("(prefers-color-scheme: dark)");

  if (localStorageTheme !== null) {
    return localStorageTheme;
  }

  if (systemSettingDark.matches) {
    return "dark";
  }

  return "light";
}
/// End Theme settings

const api = useApi();
const ffapi = useFFApi();
const title = useTitle();
const router = useRouter();
const ffstatus = useFFStatus();
const currentConfigUid = useCurrentConfigUid();
const currentFFStatus = computed(() =>
  ffstatus.value.find((s) => s.configUid === currentConfigUid.value)
);
const ffconfig = computed(() => ffapi.getConfig(currentConfigUid.value ?? ""));
const { currentRoute } = router;
const utils = useUtils();
const refreshFFStatus = ref<any>(null);
const refreshServerStatus = ref<any>(null);

//import { set } from 'video.js/dist/types/tech/middleware';
import { useTheme } from "vuetify";

function goToPrev() {
  if (currentRoute.value.name === "index") {
    return;
  }

  if (window.history?.length) {
    router.push({ name: "index" });
  } else {
    router.push({ name: "index" });
  }
}

onMounted(() => {
  ffapi.refresh_configs();
  ffapi.refresh_status();
  api.refresh_servers();
  api.refreshServerStatus();
  refreshFFStatus.value = setInterval(
    async () => await ffapi.refresh_status(),
    2000
  );
  refreshServerStatus.value = setInterval(
    async () => await api.refreshServerStatus(),
    2000
  );
});

onUnmounted(() => {
  clearInterval(refreshFFStatus.value);
  clearInterval(refreshServerStatus.value);
});
</script>
<style>
.iconify {
  font-size: 1.5em;
}
</style>
