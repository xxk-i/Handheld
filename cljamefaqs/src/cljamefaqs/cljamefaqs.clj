(ns cljamefaqs.cljamefaqs
  (:require
   [clojure.zip :as zip]
   [clj-http.client :as client]
   [hickory.core :as hick]
   [hickory.select :as s]
   [hickory.zip :as z])
  (:gen-class))

;; ffvii gamefaqs "html" walkthrough
(def test-url "https://gamefaqs.gamespot.com/ps/197341-final-fantasy-vii/faqs/71240")

(def urls {:ffvii test-url
           :ffviii "https://gamefaqs.gamespot.com/ps/197343-final-fantasy-viii/faqs/72431"
           :ocarina-of-time-3ds "https://gamefaqs.gamespot.com/n64/197771-the-legend-of-zelda-ocarina-of-time/faqs/65752"})

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println "got"))

(defn- get-url
  "Set a fake browser-looking user-agent and get the url from gamefaqs"
  [url]
  (client/get
   url
   {:headers
    {"User-Agent" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko)"}}))

(defn build-toc
  "Given a hickory data strucure representing the home page of the guide,
   we extract the table of contents based on the dom"
  [hick]
  (let [toc-links (s/select (s/and (s/descendant (s/class "ftoc") (s/tag :a))) hick)]
    (map (fn [elem]
           {:name (first (:content elem))
            :href (:href (:attrs elem))})
         toc-links)))

(defn toc
  "given a url pointing to a gamefaqs HTML guide (must be HTML), this should
   extract the table of contents from the main page"
  [url]
  (-> test-url
      get-url
      :body
      hick/parse
      hick/as-hickory
      build-toc))

(defn test-tocs [toc-map]
  (doseq [[k u] toc-map]
    (let [table (toc u)]
      (println "Looking at" k)
      (clojure.pprint/pprint (take 10 table)))))

;; "tests"
; (test-tocs urls)
; (toc test-url)
; (toc "https://gamefaqs.gamespot.com/ps/197343-final-fantasy-viii/faqs/72431")
