import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

interface IgnorePattern {
  pattern: string;
  enabled: boolean;
}

interface SettingsProps {
  onClose: () => void;
}

export const Settings = ({ onClose }: SettingsProps) => {
  const [patterns, setPatterns] = useState<IgnorePattern[]>([]);
  const [newPattern, setNewPattern] = useState("");
  const [error, setError] = useState("");

  useEffect(() => {
    loadPatterns();
  }, []);

  const loadPatterns = async () => {
    try {
      const result: IgnorePattern[] = await invoke("get_ignore_patterns");
      setPatterns(result);
    } catch (e) {
      console.error("Failed to load patterns:", e);
    }
  };

  const handleAddPattern = async () => {
    if (!newPattern.trim()) {
      setError("Pattern cannot be empty");
      return;
    }

    try {
      await invoke("add_ignore_pattern", { pattern: newPattern.trim() });
      setNewPattern("");
      setError("");
      await loadPatterns();
    } catch (e) {
      setError(String(e));
    }
  };

  const handleRemovePattern = async (pattern: string) => {
    try {
      await invoke("remove_ignore_pattern", { pattern });
      await loadPatterns();
    } catch (e) {
      console.error("Failed to remove pattern:", e);
    }
  };

  const handleTogglePattern = async (pattern: string) => {
    try {
      await invoke("toggle_ignore_pattern", { pattern });
      await loadPatterns();
    } catch (e) {
      console.error("Failed to toggle pattern:", e);
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-gray-900 rounded-lg p-6 w-full max-w-2xl max-h-[80vh] flex flex-col">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-xl font-bold text-white">Ignore Patterns</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-white text-2xl"
          >
            ×
          </button>
        </div>

        <div className="mb-4">
          <p className="text-gray-400 text-sm mb-2">
            Add patterns to ignore files and folders during scans. Use wildcards like *.log, node_modules, etc.
          </p>
          <div className="flex gap-2">
            <input
              type="text"
              value={newPattern}
              onChange={(e) => {
                setNewPattern(e.target.value);
                setError("");
              }}
              onKeyPress={(e) => e.key === "Enter" && handleAddPattern()}
              placeholder="e.g., *.log, node_modules, .git"
              className="flex-1 bg-gray-800 text-white px-3 py-2 rounded border border-gray-700 focus:border-blue-500 focus:outline-none"
            />
            <button
              onClick={handleAddPattern}
              className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
            >
              Add
            </button>
          </div>
          {error && <p className="text-red-500 text-sm mt-1">{error}</p>}
        </div>

        <div className="flex-1 overflow-y-auto">
          {patterns.length === 0 ? (
            <div className="text-gray-500 text-center py-8">
              No ignore patterns configured
            </div>
          ) : (
            <div className="space-y-2">
              {patterns.map((p) => (
                <div
                  key={p.pattern}
                  className="flex items-center gap-3 bg-gray-800 p-3 rounded"
                >
                  <input
                    type="checkbox"
                    checked={p.enabled}
                    onChange={() => handleTogglePattern(p.pattern)}
                    className="w-4 h-4"
                  />
                  <span
                    className={`flex-1 font-mono text-sm ${
                      p.enabled ? "text-white" : "text-gray-500 line-through"
                    }`}
                  >
                    {p.pattern}
                  </span>
                  <button
                    onClick={() => handleRemovePattern(p.pattern)}
                    className="px-3 py-1 text-sm text-red-400 hover:text-red-300 hover:bg-red-900 hover:bg-opacity-30 rounded transition-colors"
                  >
                    Remove
                  </button>
                </div>
              ))}
            </div>
          )}
        </div>

        <div className="mt-4 pt-4 border-t border-gray-700">
          <button
            onClick={onClose}
            className="w-full px-4 py-2 bg-gray-700 text-white rounded hover:bg-gray-600 transition-colors"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
};
