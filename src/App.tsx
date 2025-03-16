import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
// import { open } from '@tauri-apps/api/dialog';

// const selectFiles = async () => {
//   const selected = await open({
//     multiple: true,
//     filters: [{ name: 'Audio', extensions: ['mp3', 'wav', 'flac'] }],
//   });
//   console.log(selected);
// };

function App() {
  return (
    <div className="flex h-screen bg-gray-100">
      <div className="flex-1 p-8">
        <SongInfo />
        <AlbumArt />
        <ControlBar />
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

function ControlBar() {
  return (
    <div className="flex items-center justify-center mt-8">
      <Button icon="previous" />
      <Button icon="play" />
      <Button icon="next" />
      <VolumeSlider />
      <ProgressBar />
    </div>
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

export default App;
