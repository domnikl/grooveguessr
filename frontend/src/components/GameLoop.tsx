import ReactPlayer from "react-player";

type GameLoopProps = {
  lobby: any;
};

export default function GameLoop(props: GameLoopProps) {
  return <ReactPlayer url="https://www.youtube.com/watch?v=LXb3EKWsInQ" />;
}
