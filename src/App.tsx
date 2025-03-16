import { useRef, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import "./App.css";

function App() {
  return (
    <div className="flex h-screen bg-gray-100">
      <div className="flex-1 p-8">
        <h1 className="text-4xl font-bold text-gray-800">Neumorphism Player</h1>
        {/* <button onClick={() => invoke("quit")} className="bg-gray-200 p-2 rounded shadow-neumorphism">
            종료
          </button> */}
        <li style={{ listStyleType: "none" }}>음악 1</li>
        <li style={{ listStyleType: "none" }}>음악 2</li>
        <li style={{ listStyleType: "none" }}>음악 3</li>
      </div>
      <div className="flex-1 flex flex-col items-center justify-center">
        <MainPlayer />
      </div>
    </div>
  );
}

function SongInfo() {
  return (
    <div className="mb-4">
      <h2 className="text-2xl font-bold text-gray-800">곡 제목</h2>
      <p className="text-gray-600">아티스트 이름</p>
    </div>
  );
}

function AlbumArt({ url = "" }: { url?: string }) {
  let albumArtUrl = url;
  if (!albumArtUrl) albumArtUrl = reactLogo;
  return (
    <div
      className="w-64 h-64 bg-cover bg-center rounded-lg shadow-neumorphism"
      style={{ backgroundImage: `url(${albumArtUrl})` }}
    ></div>
  );
}

function Button({ icon }: { icon: "play" | "previous" | "next" }) {
  return (
    <div className="bg-gray-200 rounded-full p-4 shadow-neumorphism mx-2 active:shadow-neumorphism-active">
      {icon === "play" ? "▶" : icon === "previous" ? "⏮" : "⏭"}
    </div>
  );
}

function VolumeSlider() {
  return <input type="range" className="w-24 mx-4" />;
}

function ProgressBar() {
  return <input type="range" className="w-64 mx-4" />;
}

function MainPlayer() {
  const [songPath, setSongPath] = useState("");
  const [isPlaying, setIsPlaying] = useState(false);
  const audioRef = useRef<HTMLAudioElement>(null);

  const togglePlay = () => {
    if (isPlaying) {
      audioRef.current?.pause();
    } else {
      audioRef.current?.play();
    }
    setIsPlaying(!isPlaying);
  };

  const selectSong = async () => {
    const filePath = await open({
      filters: [{ name: "Audio", extensions: ["mp3", "wav"] }],
    });
    if (filePath) {
      setSongPath(`file://${filePath}`);
    }
  };

  return (
    <div className="flex-1 p-8">
      <button onClick={selectSong} className="mb-4 bg-gray-200 p-2 rounded shadow-neumorphism">
        곡 선택
      </button>
      <audio ref={audioRef} src={songPath} />
      <SongInfo />
      <AlbumArt />
      <ControlBar isPlaying={isPlaying} togglePlay={togglePlay} />
    </div>
  );
}

function ControlBar({ isPlaying, togglePlay }: { isPlaying: boolean; togglePlay: () => void }) {
  return (
    <div className="flex items-center justify-center mt-8">
      <Button icon="previous" />
      <Button icon={isPlaying ? "pause" : "play"} onClick={togglePlay} />
      <Button icon="next" />
      <VolumeSlider />
      <ProgressBar />
    </div>
  );
}

export default App;
