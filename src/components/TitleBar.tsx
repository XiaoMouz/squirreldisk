import Logo from "../assets/squirrel.png";
import Close from "../assets/Close.svg";
import { Link, useLocation } from "react-router-dom";
import { Platform, platform } from "@tauri-apps/api/os";
import { useEffect, useState } from "react";
import { appWindow } from "@tauri-apps/api/window";
import { Settings } from "./Settings";

const CloseButton = () => {
  return (
    <button
      onClick={() => {
        appWindow.close();
      }}
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        className="h-6 w-6"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={2}
          d="M6 18L18 6M6 6l12 12"
        />
      </svg>
    </button>
  );
};
const TitleBar = () => {
  let { state, pathname } = useLocation() as any;
  const [plf, setPlf] = useState<Platform | undefined>();
  const [showSettings, setShowSettings] = useState(false);
  
  useEffect(() => {
    platform().then((platf) => setPlf(platf));
  }, []);
  return (
    <div
      data-tauri-drag-region
      className="flex bg-darkBlue h-70 justify-between w-full items-center pl-3 pr-3 titlebar bg-cyan-800 p-2 text-white"
      style={{ background: "#0F1831" }}
    >
      {plf !== "darwin" ? (
        <img src={Logo} className="h-6 w-6"></img>
      ) : (
        <CloseButton></CloseButton>
      )}
      <div className="font-bold">
        <nav className="flex navi" aria-label="Breadcrumb">
          <ol className="inline-flex items-center space-x-1 md:space-x-3">
            <li className="inline-flex items-center">
              <Link
                to="/"
                className="inline-flex items-center text-sm  cursor-pointer text-gray-400 hover:text-white"
              >
                SquirrelDisk
              </Link>
            </li>

            {pathname == "/disk" && (
              <li>
                <div className="flex items-center">
                  <svg
                    className="w-6 h-6 text-gray-400"
                    fill="currentColor"
                    viewBox="0 0 20 20"
                    xmlns="http://www.w3.org/2000/svg"
                  >
                    <path
                      fillRule="evenodd"
                      d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
                      clipRule="evenodd"
                    />
                  </svg>
                  <Link
                    to="/"
                    className="ml-1 text-sm font-medium cursor-pointer md:ml-2 text-gray-400 hover:text-white"
                  >
                    All Disks
                  </Link>
                </div>
              </li>
            )}
            {state && state.disk && (
              <li aria-current="page">
                <div className="flex items-center">
                  <svg
                    className="w-6 h-6 text-gray-400"
                    fill="currentColor"
                    viewBox="0 0 20 20"
                    xmlns="http://www.w3.org/2000/svg"
                  >
                    <path
                      fillRule="evenodd"
                      d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
                      clipRule="evenodd"
                    />
                  </svg>
                  <span className="ml-1 text-sm font-medium md:ml-2 text-gray-500">
                    {state.isDirectory ? "Folder" : "Disk"} ({state.disk})
                  </span>
                </div>
              </li>
            )}
          </ol>
        </nav>
      </div>
      <div className="flex gap-2 items-center">
        <button
          onClick={() => setShowSettings(true)}
          className="text-gray-400 hover:text-white transition-colors"
          title="Settings"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            className="h-5 w-5"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
            />
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
            />
          </svg>
        </button>
        {plf !== "darwin" ? (
          <CloseButton></CloseButton>
        ) : (
          <img src={Logo} className="h-6 w-6"></img>
        )}
      </div>
      {showSettings && <Settings onClose={() => setShowSettings(false)} />}
    </div>
  );
};

export default TitleBar;
