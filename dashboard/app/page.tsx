"use client";

import { useEffect, useState, useCallback } from "react";
import {
  IconPlus,
  IconTrash,
  IconGripVertical,
  IconRefresh,
  IconRestore,
  IconTrashX,
  IconLoader,
  IconSun,
  IconMoon,
} from "@tabler/icons-react";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  DialogFooter,
  DialogClose,
} from "@/components/ui/dialog";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";

type Task = {
  id: string;
  title: string;
  priority: "low" | "medium" | "high";
  status: "todo" | "in-progress" | "done";
  assigned_to: string[];
  created_at: string;
  is_trash: boolean;
};

type User = {
  id: string;
  username: string;
  pic: string | null;
  created_at: string;
};

type Store = {
  tasks: Task[];
  users: User[];
};

const COLUMNS = [
  { key: "todo" as const, label: "À faire", icon: "📋" },
  { key: "in-progress" as const, label: "En cours", icon: "🔄" },
  { key: "done" as const, label: "Terminé", icon: "✅" },
];

const PRIORITY_COLORS = {
  low: "low" as const,
  medium: "medium" as const,
  high: "high" as const,
};

function PriorityBadge({ priority }: { priority: Task["priority"] }) {
  return <Badge variant={PRIORITY_COLORS[priority]}>{priority}</Badge>;
}

function daysAgo(dateStr: string): string {
  const diff = Date.now() - new Date(dateStr).getTime();
  const days = Math.floor(diff / 86400000);
  if (days === 0) return "aujourd'hui";
  if (days === 1) return "hier";
  return `il y a ${days} jours`;
}

function UserAvatar({ user }: { user: User }) {
  const initials = user.username.slice(0, 2).toUpperCase();
  return (
    <Avatar title={user.username}>
      <AvatarFallback className="text-[10px]">{initials}</AvatarFallback>
    </Avatar>
  );
}

function KanbanCard({
  task,
  users,
  onMove,
  onDelete,
  onDragStart,
}: {
  task: Task;
  users: User[];
  onMove: (id: string, status: string) => void;
  onDelete: (id: string) => void;
  onDragStart: (id: string) => void;
}) {
  const assignedUsers = users.filter((u) => task.assigned_to.includes(u.id));

  return (
    <Card
      draggable
      onDragStart={() => onDragStart(task.id)}
      className="shadow-xs hover:shadow-md transition-shadow cursor-grab active:cursor-grabbing"
    >
      <CardContent className="p-3 space-y-2">
        <div className="flex items-start justify-between gap-2">
          <div className="flex items-center gap-1.5 text-xs text-muted-foreground">
            <IconGripVertical className="h-3 w-3 shrink-0" />
            <span>{daysAgo(task.created_at)}</span>
          </div>
          <Button
            variant="ghost"
            size="icon"
            className="h-6 w-6 shrink-0 text-muted-foreground hover:text-destructive"
            onClick={() => onDelete(task.id)}
          >
            <IconTrash className="h-3.5 w-3.5" />
          </Button>
        </div>
        <p className="text-sm font-medium leading-snug">{task.title}</p>
        <div className="flex items-center justify-between gap-2">
          <PriorityBadge priority={task.priority} />
          {assignedUsers.length > 0 && (
            <div className="flex -space-x-1.5">
              {assignedUsers.map((u) => (
                <UserAvatar key={u.id} user={u} />
              ))}
            </div>
          )}
        </div>
        <div className="flex gap-1">
          {COLUMNS.filter((c) => c.key !== task.status).map((col) => (
            <Button
              key={col.key}
              variant="outline"
              size="sm"
              className="h-7 text-xs px-2 flex-1"
              onClick={() => onMove(task.id, col.key)}
            >
              {col.icon} {col.label}
            </Button>
          ))}
        </div>
      </CardContent>
    </Card>
  );
}

function AddTaskDialog({
  users,
  onAdd,
}: {
  users: User[];
  onAdd: (title: string, priority: string, assigned_to: string[]) => void;
}) {
  const [open, setOpen] = useState(false);
  const [title, setTitle] = useState("");
  const [priority, setPriority] = useState("medium");
  const [assigned, setAssigned] = useState<string[]>([]);

  const handleSubmit = () => {
    if (!title.trim()) return;
    onAdd(title.trim(), priority, assigned);
    setTitle("");
    setPriority("medium");
    setAssigned([]);
    setOpen(false);
  };

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button size="sm">
          <IconPlus className="h-4 w-4" />
          Nouvelle tâche
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Nouvelle tâche</DialogTitle>
        </DialogHeader>
        <div className="space-y-4 py-2">
          <div className="space-y-1.5">
            <label className="text-sm font-medium">Titre</label>
            <input
              className="flex h-9 w-full rounded-lg border border-input bg-transparent px-3 py-2 text-sm shadow-sm focus:outline-none focus:ring-1 focus:ring-ring"
              placeholder="Que faut-il faire ?"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              autoFocus
            />
          </div>
          <div className="space-y-1.5">
            <label className="text-sm font-medium">Priorité</label>
            <Select value={priority} onValueChange={setPriority}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="low">Low</SelectItem>
                <SelectItem value="medium">Medium</SelectItem>
                <SelectItem value="high">High</SelectItem>
              </SelectContent>
            </Select>
          </div>
          {users.length > 0 && (
            <div className="space-y-1.5">
              <label className="text-sm font-medium">Assigné à</label>
              <div className="flex flex-wrap gap-2">
                {users.map((u) => {
                  const selected = assigned.includes(u.id);
                  return (
                    <Badge
                      key={u.id}
                      variant={selected ? "default" : "outline"}
                      className="cursor-pointer"
                      onClick={() =>
                        setAssigned(
                          selected
                            ? assigned.filter((id) => id !== u.id)
                            : [...assigned, u.id],
                        )
                      }
                    >
                      {u.username}
                    </Badge>
                  );
                })}
              </div>
            </div>
          )}
        </div>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">Annuler</Button>
          </DialogClose>
          <Button onClick={handleSubmit} disabled={!title.trim()}>
            Ajouter
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

function TrashDialog({
  tasks,
  users,
  onRestore,
  onClean,
  onDrop,
}: {
  tasks: Task[];
  users: User[];
  onRestore: (id: string) => void;
  onClean: () => void;
  onDrop: (e: React.DragEvent) => void;
}) {
  const [dragOver, setDragOver] = useState(false);

  return (
    <Dialog>
      <div
        onDragOver={(e) => {
          e.preventDefault();
          setDragOver(true);
        }}
        onDragLeave={() => setDragOver(false)}
        onDrop={(e) => {
          setDragOver(false);
          onDrop(e);
        }}
        className="fixed bottom-6 right-6 z-40"
      >
        <DialogTrigger asChild>
          <Button
            size="icon"
            className={`h-14 w-14 rounded-full shadow-lg transition-all ${dragOver ? "scale-110 bg-destructive" : ""} ${tasks.length > 0 ? "ring-2 ring-destructive ring-offset-2" : ""}`}
          >
            <IconTrash className="h-6 w-6" />
          </Button>
        </DialogTrigger>
      </div>
      <DialogContent className="sm:max-w-lg">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <IconTrash className="h-5 w-5" />
            Corbeille ({tasks.length})
          </DialogTitle>
        </DialogHeader>
        {tasks.length === 0 ? (
          <p className="text-sm text-muted-foreground py-4 text-center">
            La corbeille est vide
          </p>
        ) : (
          <div className="space-y-2 max-h-80 overflow-y-auto py-2">
            {tasks.map((t) => {
              const assignedUsers = users.filter((u) =>
                t.assigned_to.includes(u.id),
              );
              return (
                <div
                  key={t.id}
                  className="flex items-center justify-between gap-3 p-2 rounded-lg border"
                >
                  <div className="flex-1 min-w-0">
                    <p className="text-sm font-medium truncate">{t.title}</p>
                    <div className="flex items-center gap-2 mt-1">
                      <PriorityBadge priority={t.priority} />
                      {assignedUsers.length > 0 && (
                        <div className="flex -space-x-1">
                          {assignedUsers.map((u) => (
                            <UserAvatar key={u.id} user={u} />
                          ))}
                        </div>
                      )}
                    </div>
                  </div>
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => onRestore(t.id)}
                  >
                    <IconRestore className="h-3.5 w-3.5 mr-1" />
                    Restaurer
                  </Button>
                </div>
              );
            })}
          </div>
        )}
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">Fermer</Button>
          </DialogClose>
          {tasks.length > 0 && (
            <Button variant="destructive" onClick={onClean}>
              <IconTrashX className="h-4 w-4 mr-1" />
              Vider la corbeille
            </Button>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

function NotInitialized({ onInit }: { onInit: () => void }) {
  const [initFolder, setInitFolder] = useState("");
  const [initLoading, setInitLoading] = useState(false);
  const [initError, setInitError] = useState<string | null>(null);
  const [initDark, setInitDark] = useState(true);

  useEffect(() => {
    fetch("/api/folder")
      .then((r) => r.json())
      .then((d) => {
        setInitFolder(d.folder);
        document.title = d.folder ? `Kanban ${d.folder}` : "Kanban";
      })
      .catch(() => {});
    const saved = localStorage.getItem("kb-theme");
    const preferred = saved ? saved === "dark" : true;
    setInitDark(preferred);
    document.documentElement.classList.toggle("dark", preferred);
  }, []);

  const toggleInitTheme = () => {
    const next = !initDark;
    setInitDark(next);
    document.documentElement.classList.toggle("dark", next);
    localStorage.setItem("kb-theme", next ? "dark" : "light");
  };

  const handleInit = async () => {
    setInitLoading(true);
    setInitError(null);
    try {
      const res = await fetch("/api/init", { method: "POST" });
      if (!res.ok) {
        const err = await res.json();
        throw new Error(err.error || "Échec de l'initialisation");
      }
      onInit();
    } catch (e: unknown) {
      setInitError(e instanceof Error ? e.message : "Erreur");
    } finally {
      setInitLoading(false);
    }
  };

  return (
    <div className="flex flex-col min-h-screen bg-zinc-50 dark:bg-zinc-950">
      <header className="sticky top-0 z-10 border-b bg-background/80 backdrop-blur-sm">
        <div className="flex items-center justify-between px-6 h-14 max-w-7xl mx-auto w-full">
          <h1 className="text-lg font-semibold tracking-tight text-primary">
            {initFolder || "Kanban"}
          </h1>
          <Button variant="ghost" size="icon" onClick={toggleInitTheme}>
            {initDark ? (
              <IconSun className="h-4 w-4" />
            ) : (
              <IconMoon className="h-4 w-4" />
            )}
          </Button>
        </div>
      </header>
      <div className="flex-1 flex items-center justify-center p-8">
        <div className="max-w-md w-full text-center space-y-6">
          <div className="rounded-xl border bg-card p-8 shadow-sm space-y-4">
            <p className="text-base">
              Ce projet n&apos;est pas encore initialisé.
            </p>
            <p className="text-sm text-muted-foreground">
              Lance{" "}
              <code className="bg-muted px-1.5 py-0.5 rounded text-xs">
                kb init
              </code>{" "}
              dans le terminal, ou clique ci-dessous pour initialiser avec les
              options par défaut.
            </p>
            {initError && (
              <p className="text-sm text-destructive bg-destructive/10 rounded-lg p-3">
                {initError}
              </p>
            )}
            <Button
              onClick={handleInit}
              disabled={initLoading}
              className="w-full"
            >
              {initLoading ? (
                <IconLoader className="h-4 w-4 animate-spin mr-2" />
              ) : (
                <IconPlus className="h-4 w-4 mr-2" />
              )}
              Initialiser le projet
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}

export default function KanbanBoard() {
  const [data, setData] = useState<Store | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [notInit, setNotInit] = useState(false);
  const [draggedId, setDraggedId] = useState<string | null>(null);
  const [folder, setFolder] = useState("");

  useEffect(() => {
    fetch("/api/folder")
      .then((r) => r.json())
      .then((d) => {
        setFolder(d.folder);
        if (d.folder) document.title = `Kanban ${d.folder}`;
      })
      .catch(() => {});
  }, []);

  const [dark, setDark] = useState(true);

  useEffect(() => {
    const saved = localStorage.getItem("kb-theme");
    const preferred = saved ? saved === "dark" : true;
    setDark(preferred);
    document.documentElement.classList.toggle("dark", preferred);
  }, []);

  const toggleTheme = () => {
    const next = !dark;
    setDark(next);
    document.documentElement.classList.toggle("dark", next);
    localStorage.setItem("kb-theme", next ? "dark" : "light");
  };

  const fetchData = useCallback(async () => {
    try {
      setLoading(true);
      setNotInit(false);
      const res = await fetch("/api/data");
      if (!res.ok) {
        const err = await res.json();
        const msg = err.error || "";
        if (msg.includes("non trouvé") || msg.includes("init")) {
          setNotInit(true);
          return;
        }
        throw new Error(msg);
      }
      const json: Store = await res.json();
      setData(json);
      setError(null);
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : "Erreur de chargement");
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchData();
  }, [fetchData]);

  if (notInit) {
    return <NotInitialized onInit={fetchData} />;
  }

  const handleMove = async (id: string, status: string) => {
    try {
      const res = await fetch("/api/move", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ id, status }),
      });
      if (!res.ok) throw new Error("Échec du déplacement");
      fetchData();
    } catch (e: unknown) {
      console.error(e);
    }
  };

  const handleDelete = async (id: string) => {
    try {
      const res = await fetch("/api/del", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ id }),
      });
      if (!res.ok) throw new Error("Échec de la suppression");
      fetchData();
    } catch (e: unknown) {
      console.error(e);
    }
  };

  const handleAdd = async (
    title: string,
    priority: string,
    assigned_to: string[],
  ) => {
    try {
      const res = await fetch("/api/add", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ title, priority, assigned_to }),
      });
      if (!res.ok) throw new Error("Échec de l'ajout");
      fetchData();
    } catch (e: unknown) {
      console.error(e);
    }
  };

  const handleRestore = async (id: string) => {
    try {
      const res = await fetch("/api/trash-restore", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ id }),
      });
      if (!res.ok) throw new Error("Échec de la restauration");
      fetchData();
    } catch (e: unknown) {
      console.error(e);
    }
  };

  const handleCleanTrash = async () => {
    try {
      const res = await fetch("/api/trash-clean", { method: "POST" });
      if (!res.ok) throw new Error("Échec du vidage");
      fetchData();
    } catch (e: unknown) {
      console.error(e);
    }
  };

  const handleColumnDrop = (status: string) => (e: React.DragEvent) => {
    e.preventDefault();
    if (draggedId) {
      handleMove(draggedId, status);
      setDraggedId(null);
    }
  };

  const handleTrashDrop = async (e: React.DragEvent) => {
    e.preventDefault();
    if (draggedId) {
      await handleDelete(draggedId);
      setDraggedId(null);
    }
  };

  const tasks = data?.tasks.filter((t) => !t.is_trash) ?? [];
  const trashTasks = data?.tasks.filter((t) => t.is_trash) ?? [];
  const users = data?.users ?? [];

  return (
    <div className="flex flex-col flex-1 min-h-screen bg-zinc-50 dark:bg-zinc-950">
      <header className="sticky top-0 z-10 border-b bg-background/80 backdrop-blur-sm">
        <div className="flex items-center justify-between px-6 h-14 max-w-7xl mx-auto w-full">
          <div className="flex items-center gap-3">
            <h1 className="text-lg font-semibold font-heading tracking-tight text-primary">
              {folder || "Kanban"}
            </h1>
          </div>
          <div className="flex items-center gap-2">
            <Button variant="ghost" size="icon" onClick={toggleTheme}>
              {dark ? (
                <IconSun className="h-4 w-4" />
              ) : (
                <IconMoon className="h-4 w-4" />
              )}
            </Button>
            <Button
              variant="ghost"
              size="icon"
              onClick={fetchData}
              disabled={loading}
            >
              <IconRefresh
                className={`h-4 w-4 ${loading ? "animate-spin" : ""}`}
              />
            </Button>
            <AddTaskDialog users={users} onAdd={handleAdd} />
          </div>
        </div>
      </header>

      <main className="flex-1 p-6 pb-24">
        {error && (
          <div className="max-w-7xl mx-auto mb-4 p-3 rounded-lg bg-destructive/10 text-destructive text-sm border border-destructive/20">
            {error}
          </div>
        )}

        <div className="max-w-7xl mx-auto">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            {COLUMNS.map((col) => {
              const colTasks = tasks.filter((t) => t.status === col.key);
              const isOver = draggedId !== null;
              return (
                <div
                  key={col.key}
                  onDragOver={(e) => {
                    e.preventDefault();
                    isOver &&
                      e.dataTransfer &&
                      (e.dataTransfer.dropEffect = "move");
                  }}
                  onDrop={handleColumnDrop(col.key)}
                  className={`flex flex-col gap-3 rounded-xl p-1 transition-colors ${isOver ? "bg-primary/5 ring-1 ring-primary/20" : ""}`}
                >
                  <div className="flex items-center gap-2 px-1">
                    <span className="text-lg">{col.icon}</span>
                    <h2 className="font-semibold text-sm">{col.label}</h2>
                    <span className="ml-auto text-xs text-muted-foreground bg-muted rounded-full px-2 py-0.5">
                      {colTasks.length}
                    </span>
                  </div>
                  <div className="flex flex-col gap-2 min-h-[200px]">
                    {colTasks.length === 0 ? (
                      <div className="flex items-center justify-center h-32 rounded-xl border-2 border-dashed border-border text-sm text-muted-foreground">
                        Aucune tâche
                      </div>
                    ) : (
                      colTasks.map((task) => (
                        <KanbanCard
                          key={task.id}
                          task={task}
                          users={users}
                          onMove={handleMove}
                          onDelete={handleDelete}
                          onDragStart={setDraggedId}
                        />
                      ))
                    )}
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      </main>

      <TrashDialog
        tasks={trashTasks}
        users={users}
        onRestore={handleRestore}
        onClean={handleCleanTrash}
        onDrop={handleTrashDrop}
      />
    </div>
  );
}
