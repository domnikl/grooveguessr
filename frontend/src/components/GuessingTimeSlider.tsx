import { Slider, Stack, Typography } from "@mui/material";
import { formatTime } from "../utils";

type GuessingTimeSliderProps = {
  guessingTime: number;
  ariaLabel: string;
  defaultValue: number;
  min: number;
  max: number;
  disabled: boolean;
  onChange: (value: number) => void;
  onChangeCommitted: (value: number) => void;
};

export default function GuessingTimeSlider(props: GuessingTimeSliderProps) {
  const mapValue = (value: number) => {
    return Math.ceil((value as number) / 10) * 10;
  };

  return (
    <Stack direction="row" sx={{ mb: 1 }} alignItems="center">
      <Stack direction="column" sx={{ width: "100%" }} className="foobar">
        <Typography>Guessing Time</Typography>

        <Stack direction="row" justifyContent="flex-end">
          <Slider
            aria-label={props.ariaLabel}
            min={props.min}
            max={props.max}
            disabled={props.disabled}
            value={props.guessingTime}
            onChange={(_, value) => {
              props.onChange(mapValue(value as number));
            }}
            onChangeCommitted={(_, value) => {
              props.onChangeCommitted(mapValue(value as number));
            }}
          />
          <Typography>{formatTime(props.guessingTime)}</Typography>
        </Stack>
      </Stack>
    </Stack>
  );
}
