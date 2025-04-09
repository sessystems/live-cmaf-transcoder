<template>
  <v-card elevation="0">
    <v-card-title> MPEG-DASH/HLS Output </v-card-title>
    <v-card-text>
      <v-row dense>
        <v-col cols="6" lg="3">
          <v-select
            v-model="config.mpdType"
            :items="mpdTypeList"
            label="Type of MPD"
            variant="underlined"
            density="compact"
          />
        </v-col>

        <v-col cols="6" lg="3">
          <v-text-field
            v-model.number="config.segmentDurationMs"
            type="number"
            label="Segment duration"
            suffix="milliseconds"
            variant="underlined"
            density="compact"
          />
        </v-col>
        <v-col cols="6" lg="3">
          <v-text-field
            v-model.number="config.windowSize"
            type="number"
            label="Timeshift buffer"
            suffix="segments"
            :hint="
              'Timeshift buffer duration = ' +
              (config.segmentDurationMs * config.windowSize) / 1000.0 +
              's'
            "
            variant="underlined"
            density="compact"
          />
        </v-col>
        <v-col cols="6" lg="3">
          <v-text-field
            v-model.number="config.astDelayMs"
            label="Availability Start Time Offset"
            suffix="milliseconds"
            hint="Increase the offset to avoid 404 when requesting the segment too early."
            variant="underlined"
            density="compact"
          />
        </v-col>
      </v-row>

      <v-row dense>
        <v-col>
          <div class="d-flex align-center">
            <v-text-field
              v-model="config.mediaSegName"
              variant="underlined"
              density="compact"
              label="Media Segment Template"
            />
            <v-menu location="left">
              <template #activator="{ props: menu }">
                <v-tooltip location="top">
                  <template #activator="{ props: tooltip }">
                    <v-btn
                      v-bind="mergeProps(menu, tooltip)"
                      size="small"
                      density="compact"
                      icon="mdi-help"
                      variant="text"
                      class="mb-2"
                    />
                  </template>
                  <span>Help</span>
                </v-tooltip>
              </template>
              <v-card max-width="500">
                <v-card-title>Media Segment Template Help</v-card-title>
                <v-card-text>
                  <v-list>
                    <v-list-item>
                      <b>$RepresentationID$</b> — Replaced by the ID of the
                      Representation.
                    </v-list-item>
                    <v-list-item>
                      <b>$Number%0Nd$</b> — Replaced by the Segment Number. The
                      optional <code>%0Nd</code> specifies the minimum number of
                      digits, padded with leading zeros.
                    </v-list-item>

                    <v-list-item>
                      <b>$Bandwidth%0Nd$</b> — Replaced by the Representation's
                      bandwidth. The optional <code>%0Nd</code> specifies the
                      minimum number of digits, padded with leading zeros.
                    </v-list-item>
                    <v-list-item>
                      <b>$Time%0Nd$</b> — Replaced by the segment start time.
                      The optional <code>%0Nd</code> specifies the minimum
                      number of digits, padded with leading zeros.
                    </v-list-item>
                    <v-list-item>
                      <b>"$ext$"</b> : Replaced with the file extension, such as
                      <code>.mp4</code>
                    </v-list-item>
                    <v-list-item>
                      <b>"$$"</b> : Replaced with a literal dollar sign
                      (<code>$</code>).
                    </v-list-item>
                  </v-list>
                </v-card-text>
              </v-card>
            </v-menu>
          </div>
        </v-col>

        <v-col>
          <div class="d-flex align-center">
            <v-text-field
              v-model="config.initSegName"
              label="Init Segment Template"
              variant="underlined"
              density="compact"
            />
            <v-menu location="left">
              <template #activator="{ props: menu }">
                <v-tooltip location="top">
                  <template #activator="{ props: tooltip }">
                    <v-btn
                      v-bind="mergeProps(menu, tooltip)"
                      size="small"
                      density="compact"
                      icon="mdi-help"
                      variant="text"
                      class="mb-2"
                    />
                  </template>
                  <span>Help</span>
                </v-tooltip>
              </template>
              <v-card max-width="500">
                <v-card-title>Init Segment Template Help</v-card-title>
                <v-card-text>
                  <v-list>
                    <v-list-item>
                      <b>$RepresentationID$</b> — Replaced by the ID of the
                      Representation.
                    </v-list-item>
                    <v-list-item>
                      <b>$Bandwidth%0Nd$</b> — Replaced by the Representation's
                      bandwidth. The optional <code>%0Nd</code> specifies the
                      minimum number of digits, padded with leading zeros.
                    </v-list-item>
                    <v-list-item>
                      <b>"$ext$"</b> : Replaced with the file extension, such as
                      <code>.mp4</code>
                    </v-list-item>
                    <v-list-item>
                      <b>"$$"</b> : Replaced with a literal dollar sign
                      (<code>$</code>).
                    </v-list-item>
                  </v-list>
                </v-card-text>
              </v-card>
            </v-menu>
          </div>
        </v-col>
      </v-row>

      <v-row dense>
        <v-col>
          <v-text-field
            v-model="config.utcTimingUrl"
            label="UTC Timing URL"
            variant="underlined"
            density="compact"
          />
        </v-col>
        <v-col>
          <v-switch
            v-model="config.enableHls"
            label="Enable HLS"
            color="info"
            density="compact"
          />
        </v-col>
      </v-row>

      <v-row dense>
        <v-col>
          <v-text-field
            v-model="config.output"
            label="Path of the Output Stream"
            :rules="[
              () => !!config.output || 'This field is required',
              () => check_output_valid() || 'This path is not valid',
            ]"
            persistent-hint
            :hint="dash_url + (config.enableHls ? ', ' + hls_url : '')"
            variant="underlined"
            density="compact"
          />
        </v-col>
        <v-col lg="6">
          <v-select
            v-model="config.serverUid"
            :items="serverList"
            label="Transcoding Server instance"
            variant="underlined"
            density="compact"
          />
        </v-col>
      </v-row>
    </v-card-text>
  </v-card>
</template>
<script setup lang="ts">
import type { FFConfig } from "~/backend/models/FFConfig";
import { mergeProps } from "vue";
const config = defineModel<FFConfig>({ required: true });
const utils = useUtils();
const servers = useServers();

const mpdTypeList = ["Template", "SegmentTimeline"];

const serverList = computed(() => {
  return servers.value.map((server) => {
    return {
      title: server.name,
      value: server.uid,
    };
  });
});

function check_output_valid() {
  return utils.check_output_path_valid(
    config.value.output,
    config.value.serverUid,
    config.value.uid
  );
}

const dash_url = computed(() => {
  return utils.get_dash_url(config.value);
});

const hls_url = computed(() => {
  return utils.get_hls_url(config.value);
});
</script>
